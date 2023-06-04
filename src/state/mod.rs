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

//! This module contains global state structure.

use crate::objects::Game;

use std::collections::LinkedList;
use std::sync::Mutex;

/// The `StateHolder` structure is responsible for maintaining and updating the state of the game.
/// It holds a list of `Game` instances and a boolean indicating if the player is in a game.
pub struct StateHolder {
    // The list of games
    games: Mutex<LinkedList<Game>>,
    // Flag saying player in game now or not
    in_game: Mutex<bool>,
}

impl StateHolder {
    /// Constructs a new empty `StateHolder` instance.
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `StateHolder`.
    pub fn new() -> Self {
        StateHolder {
            games: Mutex::new(LinkedList::new()),
            in_game: Mutex::new(false),
        }
    }

    /// Marks the player as out of game and drops the current game.
    /// NOTE: Drop does not delete the game, but clears some fields.
    ///
    /// # Arguments
    ///
    /// * Self - A current instance of `StateHolder`.
    ///
    /// # Returns
    ///
    /// * None
    pub fn leave_game(&self) {
        // Take a mutex lock
        let mut in_game = self.in_game.lock().unwrap();
        // Mark that player leave game
        *in_game = false;

        // Take a mutex lock
        let mut games = self.games.lock().unwrap();
        // Take the current game and drop it
        if let Some(game) = games.front_mut() {
            game.drop_game();
        }
    }

    /// Sets the current game and marks the player as in game.
    ///
    /// # Arguments
    ///
    /// * `game` - The current game.
    ///
    /// # Returns
    ///
    /// * None
    pub fn set_game(&self, game: Game) {
        let mut in_game = self.in_game.lock().unwrap();
        *in_game = true;

        let mut games = self.games.lock().unwrap();
        if let Some(first_game) = games.front_mut() {
            first_game.drop_game();
        }

        let existing_game = games
            .iter()
            .find(|g| g.instance_id == game.instance_id)
            .cloned();
        games.push_front(existing_game.unwrap_or(game));
    }

    /// Returns a reference to the Mutex protecting the LinkedList of games.
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Mutex protecting the LinkedList of games.
    pub fn get_games(&self) -> &Mutex<LinkedList<Game>> {
        &self.games
    }

    /// Returns true if the player is in a game, false otherwise.
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * true if the player is in a game, false otherwise.
    pub fn is_in_game(&self) -> bool {
        *self.in_game.lock().unwrap()
    }

    /// Counts the number of games ago since the current game.
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Returns Some with the number of games ago, or None if cannot calculate
    pub fn games_ago(&self) -> Option<usize> {
        let games = self.games.lock().unwrap();
        let mut counter = 0;
        let mut iterator = games.iter();
        let game = iterator.next()?;

        while let Some(next_game) = iterator.next() {
            counter += 1;
            if next_game == game {
                return Some(counter);
            }
        }
        None
    }
}
