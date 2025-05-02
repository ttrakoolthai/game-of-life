#![no_std]
#![no_main]

mod life;
use life::*;

// use panic_halt as _;
use panic_rtt_target as _;
use cortex_m_rt::entry;
// use rtt_target::{rtt_init_print, rprintln};

#[rustfmt::skip]
use microbit::{
    board::{Board, Buttons},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};

// use microbit::{
//     board::Board,
//     display::blocking::Display,
//     hal::{timer::Timer},
// };
use embedded_hal::digital::InputPin;
use embedded_hal::delay::DelayNs;

use nanorand::{Rng, pcg64::Pcg64};

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    // rprintln!("RTT started");
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut buttons = board.buttons;
    let mut rng = Pcg64::new_seed(42u128);

    let mut fb = random_frame(&mut rng);
    let mut b_ignore = 0;

    loop {
        if buttons.button_a.is_low().unwrap() {
            fb = random_frame(&mut rng);
        } else if buttons.button_b.is_low().unwrap() && b_ignore == 0 {
            complement_frame(&mut fb);
            b_ignore = 5;
        } else if done(&fb) {
            timer.delay_ms(500u32);
            if buttons.button_a.is_low().unwrap() || buttons.button_b.is_low().unwrap() {
                continue;
            }
            fb = random_frame(&mut rng);
        } else {
            life(&mut fb);
        }

        display.show(&mut timer, fb, 100);

        if b_ignore > 0 {
            b_ignore -= 1;
        }
    }
}

fn random_frame<R: Rng<8>>(rng: &mut R) -> [[u8; 5]; 5] {
    let mut frame = [[0u8; 5]; 5];
    for row in 0..5 {
        for col in 0..5 {
            frame[row][col] = (rng.generate::<u8>() & 1) as u8;
        }
    }
    frame
}

fn complement_frame(fb: &mut [[u8; 5]; 5]) {
    for row in 0..5 {
        for col in 0..5 {
            fb[row][col] = 1 - fb[row][col];
        }
    }
}
