
#![no_std]
#![no_main]
#![deny(unsafe_code)]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main () -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    
    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        rprintln!("Dark!");
        timer.delay_ms(1_000_u16);
        row1.set_high().unwrap();
        rprintln!("Light!");
        timer.delay_ms(1_000_u16);
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
// debugging
// 