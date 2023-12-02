
#![no_std]
#![no_main]
#![deny(unsafe_code)]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_halt as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

const PIXELS: [(usize, usize); 16] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
];

#[entry]
fn main () -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0, 0);
    loop {
        for current_led in PIXELS.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, leds, 300);
            last_led = *current_led;
        }
    }
}


// build 
// $ cargo build --features v2 --target thumbv7em-none-eabihf
// 
// verify
// $ cargo readobj --features v2 --target thumbv7em-none-eabihf --bin microbit-demo -- --file-headers
//
// write it to flash
// $ cargo embed --features v2 --target thumbv7em-none-eabihf
// 
// $ cargo size --features v2 --target thumbv7em-none-eabihf -- -A
// $ cargo size --features v2 --target thumbv7em-none-eabihf --release -- -A