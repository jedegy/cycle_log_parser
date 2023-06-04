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

//! `Log` is one of the widgets (blocks) of the `Overlay` component. It creates a scrolling area
//! where incoming events are displayed with a timer.

use super::super::events::Event;
use crate::objects::Game;
use crate::state::StateHolder;

/// The `Log` struct represents a log widget, maintaining a queue of events.
pub struct Log {
    /// Queue of events to be logged.
    log: std::collections::VecDeque<Box<dyn Event>>,
}

/// The `Default` implementation provides the initial state for the `Log` widget.
impl Default for Log {
    fn default() -> Self {
        Self {
            log: std::collections::VecDeque::new(),
        }
    }
}

impl Log {
    /// This method renders the `Log` widget to the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `egui::Ui` instance.
    ///
    /// # Returns
    ///
    /// * None
    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Create a vertical ScrollArea that automatically shrinks and sticks to the bottom
        egui::Frame::none().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    // Display each event in the log
                    for event in self.log.iter_mut() {
                        event.show(ui);
                    }
                });
        });
    }

    /// This method adds an event to the log.
    ///
    /// # Arguments
    ///
    /// * `event` - A `Box` containing an object implementing the `Event` trait.
    pub fn post(&mut self, event: Box<dyn Event>) {
        self.log.push_back(event)
    }

    /// This method updates the state of the `Log` widget based on the game state.
    ///
    /// # Arguments
    ///
    /// * `game` - An Option that can contain the current game state.
    /// * `_state` - A reference to the `StateHolder` which holds the state information.
    ///
    /// # Returns
    ///
    /// * None
    pub fn on_state_update(&mut self, game: Option<Game>, _state: std::sync::Arc<StateHolder>) {
        if game.is_none() {
            self.log.clear();
        }
    }
}
