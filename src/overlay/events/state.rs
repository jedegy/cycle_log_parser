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

//! This module contains non-displayed events that are needed to update the displayed data in some
//! `Overlay` widgets (blocks)

use crate::objects::Game;

/// The `TotalPlayerCountUpdate` structure represents the total player count update in the game.
/// It contains the current number of players.
#[derive(Debug)]
pub struct TotalPlayerCountUpdate {
    pub players: usize,
}

impl TotalPlayerCountUpdate {
    /// Constructs a new `TotalPlayerCountUpdate` instance.
    ///
    /// # Arguments
    ///
    /// * `players` - The current number of players.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `TotalPlayerCountUpdate`.
    pub fn new(players: usize) -> Self {
        Self { players }
    }
}

/// The `NearPlayerCountUpdate` structure represents number of enemy players near with the player
/// including players who are in a team with the current user.
#[derive(Debug)]
pub struct NearPlayerCountUpdate {
    pub players: usize,
}

/// Constructs a new `NearPlayerCountUpdate` instance.
///
/// # Arguments
///
/// * `players` - The current number of enemy players near with the player.
///
/// # Returns
///
/// * Self - A new instance of `NearPlayerCountUpdate`.
impl NearPlayerCountUpdate {
    pub fn new(players: usize) -> Self {
        Self { players }
    }
}

/// The `UpdateState` structure represents a game state update.
/// It contains the updated state of the game.
#[derive(Debug)]
pub struct UpdateState {
    pub game: Option<Game>,
}

impl UpdateState {
    /// Constructs a new `UpdateState` instance.
    ///
    /// # Arguments
    ///
    /// * `game` - The updated state of the game.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `UpdateState`.
    pub fn new(game: Option<Game>) -> Self {
        Self { game }
    }
}
