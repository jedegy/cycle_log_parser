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
//! related to the environment events in game, such as evacuation ship called or meteor event

use crate::overlay::events;
use crate::state::StateHolder;

use winit::event_loop::EventLoopProxy;

use std::sync;

/// Struct that parses game events.
pub struct Parser;

impl Default for Parser {
    /// Constructs a default Parser object.
    fn default() -> Self {
        Self
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
        // If the event is of type "LogYActivities" and the game is in progress
        if type_ == "LogYActivities" && state.is_in_game() {
            match text {
                // If the event indicates the evacuation ship being called
                t if t.starts_with("Warning: AC_EvacShip_BP") => {
                    // Lock the event loop proxy and send a `EvacShipCalled` event for `Overlay`
                    let sender = event_loop_proxy.lock().unwrap();
                    sender
                        .send_event(events::Action::EvacShipCalled(events::EvacShipCalled::new(
                            time,
                            chrono::Duration::seconds(86),
                            "Evac ship [called]".to_string(),
                        )))
                        .unwrap();

                    // Log the event
                    log::info!("Evac ship called");
                }
                // If the event indicates the start of a meteor shower
                t if t.starts_with("Warning: AA_MeteorShowerSpawner") => {
                    // Lock the event loop proxy and send a `MeteorsEvent` event for `Overlay`
                    let sender = event_loop_proxy.lock().unwrap();
                    sender
                        .send_event(events::Action::MeteorsEvent(events::MeteorsEvent::new(
                            time,
                            chrono::Duration::seconds(45),
                            "Meteors event!".to_string(),
                        )))
                        .unwrap();

                    // Log the event
                    log::info!("Meteors event!")
                }
                _ => (),
            }
        }
    }
}
