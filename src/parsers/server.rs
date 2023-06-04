// Copyright (c) 2023
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This module contains implementation of the `Parser` trait to search for events in the game log
//! related to the game, such as starting a new session, exiting it and etc.

use super::substring_between;
use crate::objects::{Game, GameMap};
use crate::overlay::events;
use crate::state::StateHolder;

use winit::event_loop::EventLoopProxy;

use std::ops::Sub;
use std::sync;

/// Parser struct that store some details about the server and current game instance,
/// and is responsible for parsing information about the server and game.
pub struct Parser {
    // Unique identifier for the game instance
    instance_id: String,
    // Geographic region of the game
    region: String,
    // Map of the game
    map: GameMap,
    // Size of the party
    party_size: usize,
    // Time when the game instance was created
    created_at: chrono::DateTime<chrono::Utc>,
    // State of the game, whether it is on hold or not
    hold: bool,
}

impl Default for Parser {
    /// Constructs a new Parser object with default values.
    fn default() -> Self {
        Self {
            instance_id: String::new(),
            region: String::new(),
            map: GameMap::TharisIsland(crate::objects::NORMAL.clone()),
            party_size: 0,
            created_at: chrono::DateTime::default(),
            hold: false,
        }
    }
}

impl super::Parser for Parser {
    /// Parse a game instance's event log from file and update the state accordingly.
    ///
    /// # Arguments
    ///
    /// * `state` - A reference to an instance of `StateHolder` shared among multiple threads.
    /// * `time` - Current UTC timestamp according the log information
    /// * `type_` - A string reference representing type of event
    /// * `text` - A string reference representing text to parse.
    /// * `event_loop_proxy` - A reference to an instance of `EventLoopProxy` shared among multiple
    /// threads, allowing safe mutation.
    ///
    /// # Returns
    ///
    /// * None
    fn parse(
        &mut self,
        state: sync::Arc<StateHolder>,
        time: chrono::DateTime<chrono::Utc>,
        type_: &str,
        text: &str,
        event_loop_proxy: sync::Arc<sync::Mutex<EventLoopProxy<events::Action>>>,
    ) {
        // Handle different types of game log events
        match type_ {
            // If the event indicates travel to a server
            "LogYTravel" => match text {
                t if t.starts_with("UYControllerTravelComponent::TravelToServer") => {
                    // Parse whether the game is a match or not
                    if let Some(result) = substring_between(t, "m_isMatch [", "]") {
                        // If the game is not a match
                        if result == "0" {
                            log::info!("--------------- LEAVE GAME ---------------");
                            // Update global state (player leaves the game)
                            state.leave_game();
                            // Send an update to the game state in `Overlay`
                            let sender = event_loop_proxy.lock().unwrap();
                            sender
                                .send_event(events::Action::UpdateState(events::UpdateState::new(
                                    None,
                                )))
                                .unwrap();
                        } else {
                            // If the game is a match
                            self.hold = true;
                            // Extract and store the instance ID and region of the game
                            self.instance_id = substring_between(t, "sessionId [", "]").unwrap();
                            self.region = substring_between(t, "region [", "]").unwrap();
                        }
                    } else {
                        // If parsing fails, log an error
                        log::error!("Cannot parse: {}", text.to_string());
                    }
                }
                // If the event indicates forced transition
                t if t.starts_with("Forcing transition to match") => {
                    // Parse and store the size of the party
                    let size = substring_between(t, "SquadSize=", "?");
                    self.party_size = size.map_or(1, |s| s.parse().unwrap());
                }
                _ => (),
            },
            // If the game is on hold and a handshake is occurring
            "LogHandshake" if self.hold && text.starts_with("SendChallengeResponse") => {
                // Calculate the creation time of the game instance
                let seconds_since_start = substring_between(text, "Timestamp: ", ".")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    - 5;
                self.created_at = time.sub(chrono::Duration::seconds(seconds_since_start));
            }
            // If the game is on hold and the player is welcomed by the server
            "LogNet" if self.hold && text.starts_with("Welcomed by server") => {
                // Parse and store the game map
                let map_s = substring_between(text, "/Game/Maps/MP/", "/").unwrap();
                let map = parse_map(map_s);
                if let Some(map) = map {
                    self.map = map;
                }

                // Create a new game instance
                let game = Game::new(
                    self.instance_id.clone(),
                    self.region.clone(),
                    self.map.clone(),
                    self.created_at,
                    self.party_size,
                );
                self.hold = false;

                // Log the new game instance
                log::info!("==================================================");
                log::info!("New instance: {:?}", game.name);
                log::info!("==================================================");

                // Update global state (started new game)
                state.set_game(game.clone());
                // Send an updated to the game state in `Overlay`
                let sender = event_loop_proxy.lock().unwrap();
                sender
                    .send_event(events::Action::UpdateState(events::UpdateState::new(Some(
                        game,
                    ))))
                    .unwrap();
            }
            _ => (),
        }
    }
}

/// Parse a string map name into a GameMap variant.
///
/// # Arguments
///
/// * `map` - String name of the map.
///
/// # Returns
///
/// * `Option<GameMap>` - Corresponding GameMap variant if the map string is recognized, None otherwise.
fn parse_map(map: String) -> Option<GameMap> {
    match map.as_str() {
        "MAP01" => Some(GameMap::BrightSands(crate::objects::NORMAL.clone())),
        "MAP02" => Some(GameMap::CrescentFalls(crate::objects::NORMAL.clone())),
        "AlienCaverns" => Some(GameMap::TharisIsland(crate::objects::THARIS.clone())),
        _ => None,
    }
}
