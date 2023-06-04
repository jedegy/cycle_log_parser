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

//! `Time` is one of the widgets (blocks) of the `Overlay` component.
//! It creates a block with timers until morning, day, evening, night and session restart.

use crate::objects::{Game, GameMap};

/// The `Time` struct represents a time widget, containing game start and end times, and associated map data.
pub struct Time {
    /// Timestamp for the start of the game.
    game_start: i64,
    /// Timestamp for the end of the game.
    game_end: i64,
    /// Optional game map information associated with the current game.
    map: Option<GameMap>,
    /// Boolean representing whether the widget is visible.
    visible: bool,
}

/// The `Default` implementation provides the initial state for the `Time` widget.
impl Default for Time {
    fn default() -> Self {
        Self {
            game_start: 0,
            game_end: 0,
            map: None,
            visible: false,
        }
    }
}

impl Time {
    /// This method renders the `Time` widget to the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `egui::Ui` instance.
    ///
    /// # Returns
    ///
    /// * None
    pub fn show(&mut self, ui: &mut egui::Ui) {
        // If no map data is present, stop the method execution.
        if self.map.is_none() {
            return;
        }

        // The current timestamp in milliseconds.
        let now = chrono::Utc::now().timestamp_millis();

        // Calculating the game time elapsed since the start.
        let mut time = now - self.game_start;
        // We clone the map and unwrap it because we have checked that it's not None.
        let map = self.map.clone().unwrap();
        // Extract timing details from the map.
        let timings = map.timings();
        // Get the remainder of time to determine the current phase (morning, day, evening, or night).
        time %= timings.time_between_storms;

        // Initializing the target time to 0.
        // This will be updated with each time phase boundary.
        let mut target = 0;

        // Calculate the time remaining until each phase of the day.
        let to_morning = Time::diff(time, target, timings.time_between_storms);
        target += timings.morning;
        let to_day = Time::diff(time, target, timings.time_between_storms);
        target += timings.day;
        let to_evening = Time::diff(time, target, timings.time_between_storms);
        target += timings.evening;
        let to_night = Time::diff(time, target, timings.time_between_storms);

        // Calculate the time remaining until the server is due to shut down.
        let to_server_death = self.game_end - now;

        // If the `Time` object is set to visible, we draw its UI elements.
        if self.visible {
            // Each time label is displayed with appropriate color coding.
            // The if-else block determines the color of the server death time label.
            // The show_label method call renders each label to the UI.
            egui::Frame::none().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        super::super::show_label(
                            ui,
                            format!(
                                "{}:{:02}",
                                (to_morning / 60000),
                                ((to_morning % 60000) / 1000)
                            ),
                            egui::Color32::from_rgb(0x00, 0xcc, 0xff),
                            egui::FontFamily::Name("MonospaceX".into()),
                            28.0,
                        );
                        super::super::show_label(
                            ui,
                            format!("/ {}:{:02}", (to_day / 60000), ((to_day % 60000) / 1000)),
                            egui::Color32::from_rgb(0xff, 0xff, 0x00),
                            egui::FontFamily::Name("MonospaceX".into()),
                            28.0,
                        );
                        super::super::show_label(
                            ui,
                            format!(
                                "/ {}:{:02}",
                                (to_evening / 60000),
                                ((to_evening % 60000) / 1000)
                            ),
                            egui::Color32::from_rgb(0xff, 0xef, 0xd5),
                            egui::FontFamily::Name("MonospaceX".into()),
                            28.0,
                        );
                        super::super::show_label(
                            ui,
                            format!(
                                "/ {}:{:02}",
                                (to_night / 60000),
                                ((to_night % 60000) / 1000)
                            ),
                            egui::Color32::from_rgb(0xff, 0x00, 0x99),
                            egui::FontFamily::Name("MonospaceX".into()),
                            28.0,
                        );

                        let color = if to_server_death
                            > chrono::Duration::milliseconds(2700000).num_milliseconds()
                        {
                            egui::Color32::from_rgb(0x99, 0x66, 0x66)
                        } else {
                            egui::Color32::RED
                        };

                        super::super::show_label(
                            ui,
                            format!(
                                "/ {}:{:02}",
                                (to_server_death / 60000),
                                ((to_server_death % 60000) / 1000)
                            ),
                            color,
                            egui::FontFamily::Name("MonospaceX".into()),
                            28.0,
                        );
                    })
                })
            });
        }
    }

    /// This method updates the state of the `Time` widget based on the game state.
    ///
    /// # Arguments
    ///
    /// * `game` - An Option that can contain the current game state.
    ///
    /// # Returns
    ///
    /// * None
    pub fn on_state_update(&mut self, game: Option<Game>) {
        // If a game state is present, update widget's data and make it visible
        if let Some(game) = game {
            self.map = Some(game.map.clone());
            self.game_start = game.created_at.timestamp_millis() - game.map.timings().morning;
            self.game_end =
                game.created_at.timestamp_millis() + chrono::Duration::hours(6).num_milliseconds();
            self.visible = true;
        } else {
            // Otherwise, hide the widget
            self.visible = false;
        }
    }

    /// This method calculates the time difference, accounting for time cycle wraparound.
    ///
    /// # Arguments
    ///
    /// * `now` - The current timestamp.
    /// * `target` - The target timestamp.
    /// * `cycle` - The length of the time cycle.
    ///
    /// # Returns
    ///
    /// * A i64 value representing the calculated time difference.
    fn diff(now: i64, target: i64, cycle: i64) -> i64 {
        let diff = target - now;

        if diff > 0 {
            diff
        } else {
            cycle + diff
        }
    }
}
