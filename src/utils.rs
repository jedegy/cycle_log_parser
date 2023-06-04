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

//! This module contains some useful functions.

/// Generate a fake name consisting of a color and an animal.
///
/// # Arguments
///
/// * `rng` - A random number generator.
///
/// # Return
///
/// This function will return a string of the format "<color> <animal>".
pub fn fake_name(mut rng: rand::rngs::StdRng) -> String {
    // Import the SliceRandom trait from the rand crate.
    use rand::seq::SliceRandom;

    // Define a list of possible colors.
    let colors = vec![
        "red", "blue", "green", "yellow", "white", "black", "cyan", "magenta", "orange", "pink",
        "purple", "brown", "lime", "olive", "maroon", "navy", "gray", "silver",
    ];

    // Define a list of possible animals.
    let animals = vec![
        "cat",
        "dog",
        "lion",
        "tiger",
        "elephant",
        "giraffe",
        "bear",
        "fox",
        "wolf",
        "hippopotamus",
        "zebra",
        "deer",
        "rabbit",
        "squirrel",
        "kangaroo",
        "koala",
        "monkey",
        "penguin",
        "dolphin",
        "whale",
        "shark",
        "crocodile",
        "turtle",
        "octopus",
    ];

    // Choose a random color and animal from the lists.
    let color = colors.choose(&mut rng).unwrap().to_string();
    let animal = animals.choose(&mut rng).unwrap().to_string();

    // Return the combined color and animal as the fake name.
    format!("{} {}", color, animal)
}

/// Emit a beep sound if less than 60 seconds have passed since a specified time.
///
/// # Arguments
///
/// * `freq` - The frequency of the beep.
/// * `duration` - The duration of the beep.
/// * `time` - The time to compare with the current time.
pub fn beep(freq: u32, duration: u64, time: chrono::DateTime<chrono::Utc>) {
    // Import necessary traits from the cpal crate.
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    // Import the Sub trait from the std::ops module.
    use std::ops::Sub;

    // If less than 60 seconds have passed since the specified time...
    if chrono::Utc::now().sub(time) < chrono::Duration::seconds(60) {
        // Set up the audio output stream.
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let config = device.default_output_config().unwrap();
        let config: cpal::StreamConfig = config.into();
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Define a function to handle errors.
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        // Set up the sine wave generator.
        let sample_duration = 1.0 / sample_rate;
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            let value = (sample_clock * freq as f32 * 2.0 * std::f32::consts::PI).sin() * 0.5;
            sample_clock = (sample_clock + sample_duration) % 1.0;
            value
        };

        // Build and start the output stream.
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        let value: f32 = next_value();
                        for sample in frame.iter_mut() {
                            *sample = value;
                        }
                    }
                },
                err_fn,
                None,
            )
            .unwrap();
        stream.play().unwrap();

        // Sleep for the duration of the beep.
        std::thread::sleep(std::time::Duration::from_millis(duration));
        // Pause the output stream.
        stream.pause().unwrap();
    }
}
