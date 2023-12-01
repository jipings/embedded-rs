
#![no_std]
#![no_main]
#![deny(unsafe_code)]

// use panic_rtt_target as _;

// use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;
use panic_halt as _;
use microbit as _;

#[entry]
fn main () -> ! {
    let _y;
    let x = 42;
    _y = x;

    loop {}
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