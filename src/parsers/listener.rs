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

//! This module contains `Listener` which starts parsing the game log.

use crate::overlay::events;
use crate::state::StateHolder;

use chrono::TimeZone;
use tokio::io::AsyncBufReadExt;

use std::sync;

// Represents a listener for parsing log.
pub struct Listener {
    /// Shared global state holder.
    state: sync::Arc<StateHolder>,
    /// Regex pattern to match each line of log.
    line_pattern: regex::Regex,
    /// Format of the datetime in log.
    format: String,
    /// Collection of parsers.
    parsers: Vec<Box<dyn super::Parser + Send>>,
}

impl Listener {
    /// Creates a new listener with given state.
    ///
    /// # Arguments
    ///
    /// * `state` - A shared reference to an instance of `StateHolder`.
    ///
    /// # Return
    ///
    /// This function will return an instance of `Listener`.
    pub fn new(state: sync::Arc<StateHolder>) -> Self {
        Self {
            state,
            line_pattern: regex::Regex::new(
                r"\[(\d{4}\.\d{2}\.\d{2}-\d{2}\.\d{2}\.\d{2}:\d{3})]\[.{3}](\w*): (.*)",
            )
            .unwrap(),
            format: String::from("%Y.%m.%d-%H.%M.%S:%3f"),
            parsers: vec![
                Box::new(super::activities::Parser::default()),
                Box::new(super::player::Parser::default()),
                Box::new(super::server::Parser::default()),
            ],
        }
    }

    /// Handles a given string and sends it to the parsers.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to be handled.
    /// * `event_loop_proxy` - A shared reference to an instance of `EventLoopProxy` which is
    /// responsible for sending events.
    ///
    /// # Return
    ///
    /// * None
    async fn handle(
        &mut self,
        string: &str,
        event_loop_proxy: sync::Arc<sync::Mutex<winit::event_loop::EventLoopProxy<events::Action>>>,
    ) {
        // Attempt to capture groups in the line with the defined regex pattern
        if let Some(captures) = self.line_pattern.captures(string) {
            // Extract and parse the timestamp
            let time = chrono::Utc
                .datetime_from_str(&captures[1], &self.format)
                .unwrap();

            // Extract the type and text from the captures
            let type_ = &captures[2];
            let text = &captures[3];

            // Parse the captured data with all parsers
            for parser in self.parsers.iter_mut() {
                parser.parse(
                    self.state.clone(),
                    time,
                    type_,
                    text,
                    event_loop_proxy.clone(),
                );
            }
        }
    }

    /// Processes a log file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the log file.
    /// * `event_loop_proxy` - A shared reference to an instance of `EventLoopProxy` which is
    /// responsible for sending events.
    ///
    /// # Return
    ///
    /// * None
    pub async fn process_log_file(
        &mut self,
        file_path: std::path::PathBuf,
        event_loop_proxy: sync::Arc<sync::Mutex<winit::event_loop::EventLoopProxy<events::Action>>>,
    ) {
        // Log the start of file processing
        log::info!("Processing log file {:?} started...", file_path.clone());

        // Attempt to open the file
        match tokio::fs::File::open(file_path).await {
            Ok(file) => {
                // Create a buffer reader for the file
                let reader = tokio::io::BufReader::new(file);
                let mut reader = tokio::io::BufReader::new(reader).lines();

                loop {
                    // Read lines from the file and process them
                    match reader.next_line().await {
                        Ok(line) => {
                            if let Some(text) = line {
                                self.handle(&text, event_loop_proxy.clone()).await;
                            } else {
                                // If there is no more line to read, pause for a moment
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                            }
                        }
                        Err(e) => log::error!("Error reading line from file: {}", e),
                    }
                }
            }
            Err(e) => log::error!("Log file not found: {}", e),
        }
    }
}
