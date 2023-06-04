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

//! This module contains maps definitions.

/// Enum representing a game maps.
#[derive(PartialEq, Clone, Debug)]
pub enum GameMap {
    /// Bright Sands map with associated timings.
    BrightSands(super::Timings),
    /// Crescent Falls map with associated timings.
    CrescentFalls(super::Timings),
    /// Tharis Island map with associated timings.
    TharisIsland(super::Timings),
}

impl GameMap {
    /// Returns the timings associated with the game map.
    ///
    /// # Return
    ///
    /// This function will return a reference to the timings of the game map.
    pub fn timings(&self) -> &super::Timings {
        match self {
            GameMap::BrightSands(timings) => timings,
            GameMap::CrescentFalls(timings) => timings,
            GameMap::TharisIsland(timings) => timings,
        }
    }
}

impl Default for GameMap {
    /// Provides a default value for `GameMap`.
    ///
    /// # Return
    ///
    /// This function will return an instance of `GameMap::BrightSands` with normal timings.
    fn default() -> Self {
        GameMap::BrightSands(super::NORMAL.clone())
    }
}
