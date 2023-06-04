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

//! This module contains displayed environment events with timer, like
//! when someone calls an evacuation ship or meteorites fall.

/// The `EvacShipCalled` struct represents an event when an evacuation ship is called in the game.
/// It contains a timer, message and color.
#[derive(Debug)]
pub struct EvacShipCalled {
    timer: super::EventTimer,
    message: String,
    color: egui::Color32,
}

impl EvacShipCalled {
    /// Defining some constant colors to be used within the struct.
    const PINK_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 175, 175);
    const LIGHT_GRAY_COLOR: egui::Color32 = egui::Color32::from_rgb(192, 192, 192);
    const GREEN_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

    /// Constructs a new `EvacShipCalled` instance.
    ///
    /// # Arguments
    ///
    /// * `time` - The start time of the event.
    /// * `duration` - The duration of the event.
    /// * `message` - The message to be displayed when the event occurs.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `EvacShipCalled`.
    pub fn new(
        time: chrono::DateTime<chrono::Utc>,
        duration: chrono::Duration,
        message: String,
    ) -> Self {
        let timer = super::EventTimer::new(time, duration);
        Self {
            timer,
            message,
            color: EvacShipCalled::GREEN_COLOR,
        }
    }
}

impl super::Event for EvacShipCalled {
    /// Displays the `EvacShipCalled` event in the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `egui::Ui` instance.
    ///
    /// # Returns
    ///
    /// * None
    fn show(&mut self, ui: &mut egui::Ui) {
        let timer = self.timer.get_remaining_time();
        if !timer.is_zero() {
            // Update message and color based on remaining time
            if timer.num_milliseconds() < chrono::Duration::seconds(76).num_milliseconds() {
                if timer.num_milliseconds() < chrono::Duration::seconds(39).num_milliseconds() {
                    self.message = "Evac ship [landed]".to_string();
                }
                if timer.num_milliseconds() < chrono::Duration::seconds(10).num_milliseconds() {
                    self.message = "Evac ship [flying]".to_string();
                    self.color = EvacShipCalled::LIGHT_GRAY_COLOR;
                }
            }

            // Show the event in the UI
            egui::Frame::none().show(ui, |ui| {
                ui.horizontal(|ui| {
                    super::super::show_label(
                        ui,
                        format!("[{:02}s]", timer.num_seconds()),
                        EvacShipCalled::PINK_COLOR,
                        egui::FontFamily::Name("MonospaceX".into()),
                        25.0,
                    );
                    super::super::show_label(
                        ui,
                        self.message.to_string(),
                        self.color,
                        egui::FontFamily::Name("MonospaceX".into()),
                        25.0,
                    );
                });
            });
        }
    }
}

/// The `MeteorsEvent` struct represents a meteor event in the game.
/// It contains a timer, a message, and a color.
#[derive(Debug)]
pub struct MeteorsEvent {
    timer: super::EventTimer,
    message: String,
    color: egui::Color32,
}

impl MeteorsEvent {
    /// Defining some constant colors to be used within the struct.
    const PINK_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 175, 175);
    const GREEN_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

    /// Constructs a new `MeteorsEvent` instance.
    ///
    /// # Arguments
    ///
    /// * `time` - The start time of the event.
    /// * `duration` - The duration of the event.
    /// * `message` - The message to be displayed when the event occurs.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `MeteorsEvent`.
    pub fn new(
        time: chrono::DateTime<chrono::Utc>,
        duration: chrono::Duration,
        message: String,
    ) -> Self {
        let timer = super::EventTimer::new(time, duration);
        Self {
            timer,
            message,
            color: MeteorsEvent::GREEN_COLOR,
        }
    }
}

impl super::Event for MeteorsEvent {
    /// Displays the `MeteorsEvent` in the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `egui::Ui` instance.
    ///
    /// # Returns
    ///
    /// * None
    fn show(&mut self, ui: &mut egui::Ui) {
        let timer = self.timer.get_remaining_time();
        if !timer.is_zero() {
            // Show the event in the UI
            egui::Frame::none().show(ui, |ui| {
                ui.horizontal(|ui| {
                    super::super::show_label(
                        ui,
                        format!("[{:02}s]", timer.num_seconds()),
                        MeteorsEvent::PINK_COLOR,
                        egui::FontFamily::Name("MonospaceX".into()),
                        25.0,
                    );
                    super::super::show_label(
                        ui,
                        self.message.to_string(),
                        self.color,
                        egui::FontFamily::Name("MonospaceX".into()),
                        25.0,
                    );
                });
            });
        }
    }
}
