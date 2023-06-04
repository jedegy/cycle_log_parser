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

//! This module contains timings for different types of maps.

/// Struct representing various timings.
#[derive(PartialEq, Clone, Debug)]
pub struct Timings {
    /// The total time between storms.
    pub time_between_storms: i64,
    /// The duration of the morning.
    pub morning: i64,
    /// The duration of the day.
    pub day: i64,
    /// The duration of the evening.
    pub evening: i64,
    /// The duration of the night.
    pub night: i64,
}

impl Timings {
    /// Creates a new `Timings` instance.
    ///
    /// # Arguments
    ///
    /// * `morning` - The duration of the morning.
    /// * `day` - The duration of the day.
    /// * `evening` - The duration of the evening.
    /// * `night` - The duration of the night.
    ///
    /// # Return
    ///
    /// This function will return an instance of `Timings`.
    fn new(morning: i64, day: i64, evening: i64, night: i64) -> Self {
        // Calculate the time between storms as the sum of all other times
        let time_between_storms = morning + day + evening + night;

        Self {
            time_between_storms,
            morning,
            day,
            evening,
            night,
        }
    }
}

lazy_static::lazy_static! {
    /// Normal timings.
    pub static ref NORMAL: Timings = Timings::new(duration(4, 0), duration(16, 40), duration(13, 20), duration(4, 40));
    /// Tharis map timings.
    pub static ref THARIS: Timings = Timings::new(duration(4, 0), duration(12, 40), duration(8, 20), duration(4, 40));
}

/// Returns the total duration in milliseconds for a given amount of minutes and seconds.
///
/// # Arguments
///
/// * `minutes` - The number of minutes.
/// * `seconds` - The number of seconds.
///
/// # Return
///
/// This function will return the total duration in milliseconds.
fn duration(minutes: u64, seconds: u64) -> i64 {
    // Convert the duration into milliseconds
    chrono::Duration::seconds((minutes * 60 + seconds) as i64).num_milliseconds()
}
