#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_halt as _;
use microbit::{
    board::Board,
    gpio::BTN_A,
    gpio::BTN_B,
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;
use core::fmt::Write;

#[entry]
fn main () -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let button_a = BTN_A::from(board.buttons.button_a);
    let button_b = BTN_B::from(board.buttons.button_b);

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut state_a_low = false;
    let mut state_b_low = false;

    loop {
        // Get button states
        let button_a_low = button_a.is_low().unwrap();
        let button_b_low = button_b.is_low().unwrap();

        if button_a_low && !state_a_low {
            write!(serial, "Button A down\r\n").unwrap();
        }
        if button_b_low && !state_b_low {
            write!(serial, "Button B down\r\n").unwrap();
        }
        if !button_a_low && state_a_low {
            write!(serial, "Button A up\r\n").unwrap();
        }
        if !button_b_low && state_b_low {
            write!(serial, "Button B up\r\n").unwrap();
        }
        // Store buttons states
        // This should not read the GPIO pins again, as the state
        // may have changed and the change will not be recorded
        state_a_low = button_a_low;
        state_b_low = button_b_low;
    }
}

