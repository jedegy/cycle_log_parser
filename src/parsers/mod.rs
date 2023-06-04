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

//! This module contains `Parser` trait and helper function to parse text from game log.

mod activities;
mod listener;
mod player;
mod server;

pub use listener::Listener;

/// Represents a trait for parsing of game log functionalities
trait Parser {
    /// Parse a game instance's event log from file and update the state accordingly.
    ///
    /// # Arguments
    ///
    /// * `state` - A reference to an instance of `StateHolder` shared among multiple threads.
    /// * `time` - Current UTC timestamp according the log information
    /// * `type_` - A string reference representing type of event
    /// * `text` - A string reference representing text to parse.
    /// * `event_loop_proxy` - A reference to an instance of `EventLoopProxy` shared among multiple
    /// threads, allowing safe mutation.
    ///
    /// # Returns
    ///
    /// * None
    fn parse(
        &mut self,
        state: std::sync::Arc<crate::state::StateHolder>,
        time: chrono::DateTime<chrono::Utc>,
        type_: &str,
        text: &str,
        sender: std::sync::Arc<
            std::sync::Mutex<winit::event_loop::EventLoopProxy<crate::overlay::events::Action>>,
        >,
    );
}

/// Returns a substring between two given substrings from a text.
///
/// # Arguments
///
/// * `text` - The string to be searched.
/// * `start` - The start delimiter. The substring starts after this delimiter.
/// * `end` - The end delimiter. The substring ends before this delimiter.
///
/// # Return
///
/// This function will return `Some(String)` if a substring is found between `start` and `end`.
/// Otherwise, it will return `None`.
fn substring_between(text: &str, start: &str, end: &str) -> Option<String> {
    // Find the start index of the 'start' delimiter in the text
    text.find(start)
        // If the start delimiter is found, find the end index of the 'end' delimiter in the text,
        // relative to the start index
        .and_then(|start_index| {
            text[start_index + start.len()..]
                .find(end)
                .map(|end_index| (start_index, end_index))
        })
        // If both delimiters are found, extract the substring between them
        .map(|(start_index, end_index)| {
            text[start_index + start.len()..start_index + start.len() + end_index].to_string()
        })
}
