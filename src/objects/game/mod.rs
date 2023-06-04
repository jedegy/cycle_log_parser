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

//! This module contains game session definitions.

mod map;
mod timings;

pub use map::GameMap;
pub use timings::{Timings, NORMAL, THARIS};

/// Struct representing a game session.
#[derive(PartialEq, Clone, Debug)]
pub struct Game {
    /// The ID of the game instance.
    pub instance_id: String,
    ///  The region of the game.
    pub region: String,
    /// The name of the game.
    pub name: String,
    /// The map of the game.
    pub map: GameMap,
    /// The time when the game session was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The size of the party.
    pub party_size: usize,
    /// The total number of players.
    pub total_players: usize,
    /// The number of nearby players.
    pub near_players: usize,
    /// The kill count of each player, stored in a HashMap where the keys are player names and the
    /// values are the corresponding kill counts.
    kill_count: std::collections::HashMap<String, usize>,
}

impl Game {
    /// Creates a new `Game` instance.
    ///
    /// # Arguments
    ///
    /// * `instance_id` - The ID of the game instance.
    /// * `region` - The region of the game.
    /// * `map` - The map of the game.
    /// * `created_at` - The time when the game was created.
    /// * `party_size` - The size of the party.
    ///
    /// # Return
    ///
    /// This function will return an instance of `Game`.
    pub fn new(
        instance_id: String,
        region: String,
        map: GameMap,
        created_at: chrono::DateTime<chrono::Utc>,
        party_size: usize,
    ) -> Self {
        // Generate a name for the game from `instance_id` and my own fake name generator.
        let name = if let Some(id) = instance_id.split('-').last() {
            if !id.is_empty() {
                let seed = u64::from_str_radix(id, 16).unwrap();
                let rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
                crate::utils::fake_name(rng).to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Create and return the game
        Self {
            instance_id,
            region,
            name,
            map,
            created_at,
            party_size,
            total_players: 0,
            near_players: 0,
            kill_count: std::collections::HashMap::new(),
        }
    }

    /// Drops the game, resetting the kill counts, total number of players, and number of nearby players.
    pub fn drop_game(&mut self) {
        // Clear the kill count
        self.kill_count.clear();
        // Reset player counts
        self.total_players = 0;
        self.near_players = 0;
    }

    pub fn kill(&mut self, id: String) -> usize {
        let count = self.kill_count.entry(id).or_insert(0);
        *count += 1;
        *count
    }
}
