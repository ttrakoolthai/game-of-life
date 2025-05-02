#![no_main]
#![no_std]

// Conway's Game of Life Implementation
mod life;
use life::*;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
use nanorand::{Rng, SeedableRng, pcg64::Pcg64};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

/// The program entry point for the micro:bit-based Game of Life simulation.
///
/// Initializes peripherals, display, and timer. Seeds the software RNG using
/// entropy from the hardware RNG.
///
/// The game runs at 10 frames per second with the following options:
///     * Button A randomizes the board,
///     * Button B complements it
///
/// The board resets after going completely dark.
#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Game of Life Started");

    let board = Board::take().unwrap();
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut b_ignore_timer = 0;
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let mut hw_rng = HwRng::new(board.RNG);
    let mut rng = Pcg64::new_seed(1);
    reseed(&mut rng, &mut hw_rng);

    let mut grid = randomized_frame(&mut rng);

    loop {
        if button_a.is_low().unwrap() {
            rprintln!("Button A");
            grid = randomized_frame(&mut rng);
        } else if button_b.is_low().unwrap() && b_ignore_timer == 0 {
            rprintln!("Button B");
            complemented_frame(&mut grid);
            b_ignore_timer = 5;
            rprintln!("Ignore B for {} seconds", b_ignore_timer);
        } else if done(&grid) {
            timer.delay_ms(500);
            if button_a.is_low().unwrap() || button_b.is_low().unwrap() {
                continue;
            }
            grid = randomized_frame(&mut rng);
        } else {
            life(&mut grid);
        }

        display.show(&mut timer, grid, 100);

        if b_ignore_timer > 0 {
            b_ignore_timer -= 1;
            rprintln!("Ignore B for {} seconds", b_ignore_timer);
        }
    }
}

/// Re-seeds the software RNG using entropy from the nRF52833's hardware RNG.
/// Source: https://github.com/pdx-cs-rust-embedded/mb2-rng
fn reseed(sw_rng: &mut Pcg64, hw_rng: &mut HwRng) {
    let mut seed = [0; 16];
    hw_rng.random(&mut seed);
    sw_rng.reseed(seed);
}

/// Generates a new randomized 5×5 frame using the given RNG.
///
/// Each cell has a 50% chance of being alive (`1`) or dead (`0`).
///
/// # Returns
/// A 5×5 array of `u8` values (`0` or `1`)
fn randomized_frame<R: Rng<8>>(rng: &mut R) -> [[u8; 5]; 5] {
    let mut frame = [[0; 5]; 5];
    for row in frame.iter_mut() {
        for cell in row.iter_mut() {
            *cell = rng.generate::<u8>() & 1;
        }
    }
    frame
}

/// Inverts each cell in the 5×5 grid: alive (`1`) becomes dead (`0`), and vice
/// versa.
///
/// # Arguments
/// * `grid` - A mutable reference to a 5×5 array representing the board state.
fn complemented_frame(grid: &mut [[u8; 5]; 5]) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 1 - *cell;
        }
    }
}
