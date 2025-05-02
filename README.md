# Game-of-Life

**Author**: Tommy Trakoolthai

## Overview

This project implements Conway's Game of Life on the micro:bit v2 using (embedded)
Rust. It is simulated using a 5×5 LED grid display and runs the game with interactive
input.

### Controls

- **Button A**: Generates a new random board each frame (while held).
- **Button B**: Inverts the board (on ↔ off), creating a `complemented` board. The
  `B` button is then ignored for the next `5` frames.
- **Auto-reset**: If the board is empty, the board is auto-randomized after 0.5s or
  until a button is pressed.

### Features

- Uses the NRF52833's **hardware RNG** to generate a 'random' seed for the software
- Uses RTT output for real-time debug prints
- Runs at 10 FPS (updates every 100 ms).

### Write-up

For this project, I began by reviewing the example projects we wrote in class. I initialized the project using the provided `mb2-template-main` as a template.

Initially, I implemented the program without randomness or abstractions—making sure that the LED grid could be correctly updated. With an implementation I was confident in, I then introduced functions to encapsulate some of the program's logic.

While development went relatively smoothly, the main challenge I encountered was flashing the compiled program from my ARM-based MacBook. I had some issues with probe-rs and the linker configuration. However, the class Zulip helped me troubleshoot and gave me lots of resources. I resolved the issue by explicitly adding the correct linker flags (-C link-arg=-Tlink.x) and adjusting the Embed.toml and .cargo/config.toml accordingly.

One interesting idea I’d like to explore in future projects is using the micro:bit’s built-in accelerometer to reset the board when shaken. This could provide a more interactive user experience beyond the current button-based controls.

Overall, it was a fun project, and I hope to do more with it!
---

## Build & Flash Instructions

### 1. **Set up the Rust environment**
```bash
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install probe-rs cargo-embed
```

### 2. **Build the firmware**
cargo build --release --target thumbv7em-none-eabihf

### 3. **Flash the program to the micro:bit**
cargo embed --release

### 4. **Run the program**
cargo run --release

# Sources
https://docs.rust-embedded.org/discovery/microbit/
https://github.com/pdx-cs-rust-embedded
https://relm4.org/docs/next/nanorand/rand/trait.SeedableRng.html

ChatGPT for issues with code and to help with documentation.
