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

//! `Server` is one of the widgets (blocks) of the `Overlay` component.
//! It creates several labels where it displays server information and the number of players.

use crate::objects::Game;
use crate::state::StateHolder;

/// The `Server` struct represents a server widget, maintaining server and player info.
pub struct Server {
    /// The total number of players on the server.
    pub total_players: usize,
    /// The number of players near the user.
    pub near_players: usize,
    /// The number of party size.
    pub party_size: usize,
    /// Name of the current game session.
    pub session_name: String,
    /// Visibility of the widget.
    pub visible: bool,
}

/// The `Default` implementation provides the initial state for the `Server` widget.
impl Default for Server {
    fn default() -> Self {
        Self {
            total_players: 0,
            near_players: 0,
            party_size: 0,
            session_name: String::new(),
            visible: false,
        }
    }
}

impl Server {
    /// Constant defining the orange color used in the widget
    const ORANGE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 128, 0);

    /// This method renders the `Server` widget to the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `egui::Ui` instance.
    ///
    /// # Returns
    ///
    /// * None
    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Check widget visibility
        if self.visible {
            // Frame for player information
            egui::Frame::none().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // Displaying total players label
                        super::super::show_label(
                            ui,
                            format!("PLAYERS: TOTAL {}", self.total_players),
                            Server::ORANGE_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            40.0,
                        );
                        // Adding space for aesthetic purposes
                        ui.add_space(20.0);
                        // Displaying delimiter
                        super::super::show_label(
                            ui,
                            "|".to_string(),
                            Server::ORANGE_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            40.0,
                        );
                        // Adding space for aesthetic purposes
                        ui.add_space(20.0);
                        // Displaying near players label
                        super::super::show_label(
                            ui,
                            format!("NEAR {}", self.near_players),
                            Server::ORANGE_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            40.0,
                        );
                    })
                })
            });
            // Frame for server information
            egui::Frame::none().show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Displaying server session name label
                    super::super::show_label(
                        ui,
                        format!("SERVER: {}", self.session_name.clone()),
                        Server::ORANGE_COLOR,
                        egui::FontFamily::Name("MonospaceX".into()),
                        30.0,
                    );
                    if self.party_size > 1 {
                        // Adding space for aesthetic purposes
                        ui.add_space(20.0);
                        // Displaying the size of party
                        super::super::show_label(
                            ui,
                            format!("PARTY: {:02}", self.party_size),
                            Server::ORANGE_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            30.0,
                        );
                    }
                });
            });
        }
    }

    /// This method updates the state of the `Server` widget based on the game state.
    ///
    /// # Arguments
    ///
    /// * `game` - An Option that can contain the current game state.
    /// * `state` - A reference to the `StateHolder` which holds the state information.
    ///
    /// # Returns
    ///
    /// * None
    pub fn on_state_update(&mut self, game: Option<Game>, state: std::sync::Arc<StateHolder>) {
        // If a game state is present
        if let Some(game) = game {
            // Update party size
            self.party_size = game.party_size;

            // If the number of games ago is available
            if let Some(num_games) = state.games_ago() {
                self.session_name = format!("{} | {}", game.name, num_games);
            } else {
                self.session_name = game.name;
            }
            // Set widget visibility to true
            self.visible = true;
        } else {
            // Set widget visibility to false
            self.visible = false;
        }
    }
}
