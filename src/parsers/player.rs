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
//! related to the players activity

use super::substring_between;
use crate::objects::{Actor, Weapon};
use crate::overlay::events;
use crate::state::StateHolder;

use winit::event_loop::EventLoopProxy;

use std::sync;

/// Struct that parses game events.
pub struct Parser {
    /// Boolean indicating if the last game has finished.
    last_finished: bool,
}

impl Default for Parser {
    /// Constructs a new Parser object with default value.
    fn default() -> Self {
        Self {
            last_finished: false,
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
        // If game is not in progress, return early
        if !state.is_in_game() {
            return;
        }
        // Process the event based on its type
        match type_ {
            // Parse player-related events
            "LogYPlayer" => {
                // Get the current game state.
                let games = state.get_games();
                let mut games = games.lock().unwrap();
                let game = games.front_mut();
                if let Some(game) = game {
                    // Handle player events based on the event's text.
                    match text {
                        // When a player's state updates in the match
                        t if t.starts_with("OnRep_PlayerMatchState") => {
                            if let Some(player_state) = substring_between(t, "[", "]") {
                                // If the player has joined the match
                                if player_state == "inMatch" {
                                    // Increase number of total players in current game
                                    game.total_players += 1;
                                    // Issue beep to notify user about this event
                                    if game.total_players > game.party_size {
                                        crate::utils::beep(2000, 250, time);
                                    }
                                    self.last_finished = false;
                                } else {
                                    // If the player has left the match
                                    if game.total_players > 0 {
                                        // Decrease number of total players in current game
                                        game.total_players -= 1;
                                        // Issue beep to notify user about this event
                                        if game.total_players > game.party_size {
                                            crate::utils::beep(400, 150, time);
                                        }
                                        self.last_finished = true;
                                    }
                                }
                                // Send an event to update the total player count in `Overlay`
                                let sender = event_loop_proxy.lock().unwrap();
                                sender
                                    .send_event(events::Action::TotalPlayerCountUpdate(
                                        events::TotalPlayerCountUpdate::new(game.total_players),
                                    ))
                                    .unwrap();
                            } else {
                                log::error!("Games list is empty");
                            }
                        }
                        // When a near enemy player's state changes in the game
                        t if t.starts_with("OnPlayerStateChanged") => {
                            // Increase the number of near players
                            game.near_players += 1;
                            // Send an event to update the near player count in 'Overlay`
                            let sender = event_loop_proxy.lock().unwrap();
                            sender
                                .send_event(events::Action::NearPlayerCountUpdate(
                                    events::NearPlayerCountUpdate::new(game.near_players),
                                ))
                                .unwrap();
                        }
                        // When a near enemy player's character is destroyed
                        t if t.starts_with("AYPlayerCharacter::Destroyed()") => {
                            // Decrease the number of near players
                            if game.near_players > 0 {
                                game.near_players -= 1;
                                // Send an event to update the near player count in 'Overlay`
                                let sender = event_loop_proxy.lock().unwrap();
                                sender
                                    .send_event(events::Action::NearPlayerCountUpdate(
                                        events::NearPlayerCountUpdate::new(game.near_players),
                                    ))
                                    .unwrap();
                            }
                        }
                        // When a player's match finishes with a result
                        t if t.starts_with("AYPlayerState::OnRep_PlayerMatchFinishedResult") => {
                            // Handle match result (escaped, dead)
                            if let Some(result) = substring_between(text, "Result:", " ") {
                                match result.to_lowercase().as_str() {
                                    // If the player escaped, send event to 'Overlay'
                                    "escaped" => {
                                        let sender = event_loop_proxy.lock().unwrap();
                                        sender
                                            .send_event(events::Action::PlayerEscaped(
                                                events::PlayerEscaped::new(
                                                    time,
                                                    chrono::Duration::seconds(15),
                                                    "Player escaped".to_string(),
                                                ),
                                            ))
                                            .unwrap();
                                        log::info!("Player escaped");
                                    }
                                    // If the player died
                                    "dead" => {
                                        // Handling death cause and damage here
                                        let causer_parts =
                                            substring_between(text, "Damage:Causer:", " ").unwrap();
                                        let causer_parts_spited: Vec<&str> =
                                            causer_parts.split("_C_").collect();
                                        let causer_string = causer_parts_spited[0].to_string();
                                        let causer = Actor::get(causer_string);

                                        let origin_string =
                                            substring_between(text, "Origin:OriginRow:[", "]");
                                        let origin_weapon = origin_string.as_ref().map_or_else(
                                            || {
                                                log::error!("Origin string is empty");
                                                None
                                            },
                                            |s| Weapon::get(s.as_str()),
                                        );
                                        let damage =
                                            substring_between(text, "m_healthDamage:", " ")
                                                .unwrap()
                                                .parse::<f32>()
                                                .unwrap();

                                        let causer_kills =
                                            game.kill(causer_parts_spited[1].to_string());
                                        let weapon =
                                            causer.clone().and_then(|c| match c.name.as_str() {
                                                "None" => Weapon::get("Suicide"),
                                                "Player" => {
                                                    if origin_weapon
                                                        .as_ref()
                                                        .map(|w| w.name.as_str())
                                                        == Some("None")
                                                    {
                                                        Weapon::get("Fall")
                                                    } else {
                                                        origin_weapon
                                                    }
                                                }
                                                _ => None,
                                            });

                                        // Log this event
                                        log::info!("Player dead");
                                        log::info!("----- Killed by: {:?}", causer);
                                        log::info!("----- Weapon: {:?}", weapon);
                                        log::info!("----- Damage: {:?}", damage);
                                        log::info!("----- Causer kills {:?} times", causer_kills);

                                        // Send an 'Overlay` event to indicate that the player has died
                                        let sender = event_loop_proxy.lock().unwrap();
                                        sender
                                            .send_event(events::Action::PlayerDead(
                                                events::PlayerDead::new(
                                                    time,
                                                    chrono::Duration::seconds(15),
                                                    causer,
                                                    causer_kills,
                                                    weapon,
                                                    damage,
                                                ),
                                            ))
                                            .unwrap();
                                    }
                                    _ => {
                                        log::error!("Unknown result: {}", text.to_string());
                                    }
                                }
                            } else {
                                log::error!("Cannot parse: {}", text.to_string());
                            }
                        }
                        _ => (),
                    }
                } else {
                    log::error!("Games list is empty");
                }
            }
            // If the event type is "LogYInventory" and the last player has finished
            "LogYInventory" if self.last_finished => {
                if text.starts_with(
                    "GetInventoryComponentManager | Could not retrieve YGameStateMatch!",
                ) {
                    if let Some(game) = state.get_games().lock().unwrap().front_mut() {
                        // Increase number of total players in current game
                        game.total_players += 1;
                        self.last_finished = false;
                        // Send an event to update the total player count in `Overlay`
                        let sender = event_loop_proxy.lock().unwrap();
                        sender
                            .send_event(events::Action::TotalPlayerCountUpdate(
                                events::TotalPlayerCountUpdate::new(game.total_players),
                            ))
                            .unwrap();
                        log::info!("Player finished before loading, revert player count.");
                    }
                }
            }
            _ => (),
        }
    }
}
