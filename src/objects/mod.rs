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

//! This module contains all objects in the game.

mod actors;
mod game;
mod weapons;

pub use actors::Actor;
pub use game::{Game, GameMap, Timings, NORMAL, THARIS};
pub use weapons::Weapon;

/// Enum representing the rarity of a game item.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Rarity {
    /// The most basic rarity.
    Common,
    /// A step above common.
    Uncommon,
    /// Better than uncommon.
    Rare,
    // Even more rare.
    Epic,
    /// Among the rarest of items.
    Exotic,
    /// The second most rare items.
    Legendary,
    /// The most rare items.
    Rainbow,
}

// Implement the From trait to convert Rarity to egui::Color32.
impl From<Rarity> for egui::Color32 {
    /// Convert a Rarity value to a Color32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The Rarity to convert.
    ///
    /// # Return
    ///
    /// This function will return a Color32 value corresponding to the Rarity.
    fn from(value: Rarity) -> Self {
        match value {
            Rarity::Common => egui::Color32::from_rgb(0x97, 0x9a, 0x9a),
            Rarity::Uncommon => egui::Color32::from_rgb(0x58, 0xd6, 0x8d),
            Rarity::Rare => egui::Color32::from_rgb(0x04, 0x95, 0xb4),
            Rarity::Epic => egui::Color32::from_rgb(0xb5, 0x84, 0xc8),
            Rarity::Exotic => egui::Color32::from_rgb(0xe7, 0x4c, 0x3c),
            Rarity::Legendary => egui::Color32::from_rgb(0xff, 0x80, 0x80),
            Rarity::Rainbow => egui::Color32::GOLD,
        }
    }
}
