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

//! This module contains player displayed events with timer, like when some player exits or dies.

/// The `PlayerEscaped` struct represents a player escape event in the game.
/// It contains a timer, a message, and a color.
#[derive(Debug)]
pub struct PlayerEscaped {
    timer: super::EventTimer,
    message: String,
    color: egui::Color32,
}

impl PlayerEscaped {
    /// Defining a constant color to be used within the struct.
    const GREEN_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

    /// Constructs a new `PlayerEscaped` instance.
    ///
    /// # Arguments
    ///
    /// * `time` - The start time of the event.
    /// * `duration` - The duration of the event.
    /// * `message` - The message to be displayed when the event occurs.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `PlayerEscaped`.
    pub fn new(
        time: chrono::DateTime<chrono::Utc>,
        duration: chrono::Duration,
        message: String,
    ) -> Self {
        let timer = super::EventTimer::new(time, duration);
        Self {
            timer,
            message,
            color: PlayerEscaped::GREEN_COLOR,
        }
    }
}
impl super::Event for PlayerEscaped {
    /// Displays the `PlayerEscaped` event in the UI.
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
            super::super::show_label(
                ui,
                self.message.to_string(),
                self.color,
                egui::FontFamily::Name("MonospaceX".into()),
                25.0,
            );
        }
    }
}

/// The `PlayerDead` struct represents a player death event in the game.
/// It contains a timer, the actor causing the death, the weapon used, and the damage caused.
#[derive(Debug)]
pub struct PlayerDead {
    timer: super::EventTimer,
    actor: Option<crate::objects::Actor>,
    actor_kills: usize,
    weapon: Option<crate::objects::Weapon>,
    damage: f32,
}

impl PlayerDead {
    /// Defining a constant color to be used within the struct.
    const GREEN_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

    /// Constructs a new `PlayerDead` instance.
    ///
    /// # Arguments
    ///
    /// * `time` - The start time of the event.
    /// * `duration` - The duration of the event.
    /// * `actor` - The actor causing the player death.
    /// * `actor_kills` - The number of kills of actor.
    /// * `weapon` - The weapon used to cause the death.
    /// * `damage` - The amount of damage caused.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `PlayerDead`.
    pub fn new(
        time: chrono::DateTime<chrono::Utc>,
        duration: chrono::Duration,
        actor: Option<crate::objects::Actor>,
        actor_kills: usize,
        weapon: Option<crate::objects::Weapon>,
        damage: f32,
    ) -> Self {
        let timer = super::EventTimer::new(time, duration);
        Self {
            timer,
            actor,
            actor_kills,
            weapon,
            damage,
        }
    }
}
impl super::Event for PlayerDead {
    /// Displays the `PlayerDead` event in the UI.
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
        let actor = self.actor.clone();
        let weapon = self.weapon.clone();
        let damage = self.damage;

        if !timer.is_zero() {
            egui::Frame::none().show(ui, |ui| {
                let (message, color) = if let Some(actor) = actor {
                    (
                        format!("{} ", actor.name),
                        egui::Color32::from(actor.rarity),
                    )
                } else {
                    ("Something".to_string(), PlayerDead::GREEN_COLOR)
                };
                ui.horizontal(|ui| {
                    super::super::show_label(
                        ui,
                        message,
                        color,
                        egui::FontFamily::Name("MonospaceX".into()),
                        25.0,
                    );
                    super::super::show_label(
                        ui,
                        "kills player (".to_string(),
                        PlayerDead::GREEN_COLOR,
                        egui::FontFamily::Name("MonospaceX".into()),
                        25.0,
                    );
                    if let Some(weapon) = weapon {
                        super::super::show_label(
                            ui,
                            weapon.name.to_string(),
                            egui::Color32::from(weapon.rarity),
                            egui::FontFamily::Name("MonospaceX".into()),
                            25.0,
                        );
                        super::super::show_label(
                            ui,
                            format!(": {:02.02} )", damage),
                            PlayerDead::GREEN_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            25.0,
                        );
                    } else {
                        super::super::show_label(
                            ui,
                            format!("{:02.02} )", damage),
                            PlayerDead::GREEN_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            25.0,
                        );
                    }
                    if self.actor_kills > 1 {
                        super::super::show_label(
                            ui,
                            format!("[x{:02.02}]", self.actor_kills),
                            PlayerDead::GREEN_COLOR,
                            egui::FontFamily::Name("MonospaceX".into()),
                            25.0,
                        );
                    }
                });
            });
        }
    }
}
