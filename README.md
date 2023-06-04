# The Cycle Frontier UI Helper

## About
This is a simple UI application, entirely written in `Rust`, for `The Cycle Frontier` game - my small side project.

Utilizing the game's log, this application provides useful information on the screen during the game.
![img.png](/images/img2.png)
![img2.png](/images/img.png)

## Features

- Number of players in the current session 
- Number of players near your location
- Time until various times of day in the current session. This data can be particularly handy for tracking the beginning and end of in-game storms. 
- Time until the session restarts
- Various game events such as:
  - Killing players
  - Evacuation ship calling
  - Meteorite events

There is also a display of the session ID in a convenient form and the size of the group if you are not playing alone.

The widget window is designed to be transparent to not obstruct the view of your gameplay.

**Note:** The number of players near your location includes teammates; for instance, 
if you are playing with a friend and the number of players near you is 1, this indicates there is no one nearby besides your friend.

## Building the Application

To build the application, simply run the following command in the project root:

```
cargo build --release
```

## How to Use
After building, run the cycle_log_parser.exe located in the /target/release directory.
You can also set a different size for the widget window by specifying the width and height when calling the exe file through the console:

```
./cycle_log_parser.exe <width> <height>
```

Remember to replace `<width>` and `<height>` with your desired window dimensions.

**Note:** Ensure that the game is running and generating logs for the application to function properly.