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

//! Main module of the application. It creates and runs the graphical interface,
//! and also starts parsers in a parallel asynchronous thread.

mod objects;
mod overlay;
mod parsers;
mod state;
mod utils;

use overlay::Overlay;

use log::{error, info};
use std::{env, path::PathBuf};

/// Entry point for the application.
///
/// This function initializes the logger, verifies the existence of the game log,
/// starts the log parsers, and prepares the graphical interface.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * None
#[tokio::main]
async fn main() {
    // Initializes the logger
    env_logger::init();

    // Gets the path to the game log
    let log_path = get_log_path();
    info!("Game logs path: {:?}", log_path);

    // Checks if the game log exists
    if !log_path.exists() {
        error!("Game log doesn't exist!");
        std::process::exit(-1);
    }

    info!("Starting log parsers...");

    // Parses the command-line arguments, or uses default values
    let args: Vec<String> = env::args().collect();
    let (width, height) = if args.len() == 3 {
        (
            args[1].parse::<f32>().unwrap(),
            args[2].parse::<f32>().unwrap(),
        )
    } else {
        (800.0, 600.0)
    };

    // Creates the shared application state
    let state = std::sync::Arc::new(state::StateHolder::new());

    // Initializes the event loop
    let event_loop =
        winit::event_loop::EventLoopBuilder::<overlay::events::Action>::with_user_event().build();
    let event_loop_proxy = std::sync::Arc::new(std::sync::Mutex::new(event_loop.create_proxy()));

    // Initializes the parser listener
    let mut listener = parsers::Listener::new(state.clone());

    // Creates a new runtime for the parser
    let parser_runtime = tokio::runtime::Runtime::new().unwrap();

    // Starts the parser in the runtime
    let parser_handle = parser_runtime
        .spawn(async move { listener.process_log_file(log_path, event_loop_proxy).await });

    // Initializes and runs the graphical interface
    let overlay = Overlay::new(width, height, state);
    overlay.run(event_loop).await;

    // Aborts the parser thread when the GUI closes
    parser_handle.abort();
}

/// This function retrieves the path to the game log file.
///
/// It expects to find the LOCALAPPDATA environment variable on the system
/// and appends to it the relative path to the game log file.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * A `PathBuf` that represents the absolute path to the game log file.
fn get_log_path() -> PathBuf {
    // Gets the path to the local application data directory
    let mut prospect = env::var("LOCALAPPDATA").expect("Could not find LOCALAPPDATA");

    // Appends the path to the game log file
    prospect.push_str("\\Prospect\\Saved\\Logs\\Prospect.log");

    // Returns the full path to the game log file
    PathBuf::from(prospect)
}
