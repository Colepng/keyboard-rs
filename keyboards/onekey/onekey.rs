#![no_std]
#![no_main]

use keyboardrs::config::*;
use keyboardrs::hardware::Encoder;
use keyboardrs::keycode::{Keycodes, Keycodes::*};
use keyboardrs::{init, matrix_scaning};

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 1;
    const NUMOFROW: usize = 1;
    const NUMOFLAYES: usize = 2;
    const KEYS: [[[Keycodes; NUMOFCOL]; NUMOFROW]; NUMOFLAYES] = [[[KC_MUTE]], [[KC_MUTE]]];

    let (pins, watchdog, delay) = init();

    let col: [DynPin; NUMOFCOL] = [pins.gpio27.into()];
    let row: [DynPin; NUMOFROW] = [pins.gpio18.into()];

    let config = Config { encoder: true };

    let encoder = Encoder {
        channel_a: pins.gpio22.into(),
        channel_b: pins.gpio21.into(),
        action_clock_wise: KC_VOLUP,
        action_counter_clock_wise: KC_VOLDOWN,
    };

    matrix_scaning(col, row, KEYS, Some(encoder), config, watchdog, delay);
}
