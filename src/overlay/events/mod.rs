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

//! This module contains code related to various game events.

mod environment;
mod players;
mod state;

pub use environment::{EvacShipCalled, MeteorsEvent};
pub use players::{PlayerDead, PlayerEscaped};
pub use state::{NearPlayerCountUpdate, TotalPlayerCountUpdate, UpdateState};

use std::fmt::Debug;
use std::ops::Sub;

/// The Event trait defines an interface for all game events with timer that should be displayed in the event log.
/// All these game events should implement this trait, allowing them to be displayed in the game's UI.
pub trait Event: Debug + Send {
    fn show(&mut self, ui: &mut egui::Ui);
}

/// The `Action` enum represents a generic game action.
#[derive(Debug)]
pub enum Action {
    PlayerDead(PlayerDead),
    PlayerEscaped(PlayerEscaped),
    EvacShipCalled(EvacShipCalled),
    MeteorsEvent(MeteorsEvent),
    TotalPlayerCountUpdate(TotalPlayerCountUpdate),
    NearPlayerCountUpdate(NearPlayerCountUpdate),
    UpdateState(UpdateState),
}

/// The `EventTimer` struct represents a timer for game events.
/// It stores an end time for the event.
#[derive(Debug)]
struct EventTimer {
    end_time: chrono::DateTime<chrono::Utc>,
}

impl EventTimer {
    /// Creates a new EventTimer.
    ///
    /// # Arguments
    ///
    /// * `start_time` - The starting time of the event.
    /// * `duration` - The duration of the event.
    ///
    /// # Returns
    ///
    /// * None
    fn new(start_time: chrono::DateTime<chrono::Utc>, duration: chrono::Duration) -> Self {
        EventTimer {
            end_time: start_time + duration,
        }
    }

    /// Gets the remaining time for this event.
    /// If the event has already ended, it returns zero.
    fn get_remaining_time(&self) -> chrono::Duration {
        if chrono::Utc::now().sub(self.end_time) > chrono::Duration::zero() {
            chrono::Duration::zero()
        } else {
            self.end_time.sub(chrono::Utc::now())
        }
    }
}
