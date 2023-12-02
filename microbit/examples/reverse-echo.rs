
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::blocking::serial;
use microbit::hal::Timer;
use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;
use core::fmt::Write;
use heapless::Vec;

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main () -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        if byte == 13 {
            for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                nb::block!(serial.write(*byte)).unwrap();
                nb::block!(serial.flush()).unwrap()
            }
            buffer.clear();
        } else {
            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                buffer.clear();
            };
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