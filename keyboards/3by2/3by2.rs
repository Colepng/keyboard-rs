#![no_std]
#![no_main]

use keyboardrs::keycode::{Keycodes, Keycodes::*};
use keyboardrs::{init, matrix_scaning};

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 3;
    const NUMOFROW: usize = 2;
    const NUMOFLAYES: usize = 3;

    #[rustfmt::skip]
    const KEYS: [[[Keycodes; NUMOFCOL]; NUMOFROW]; NUMOFLAYES] = [
        [[KC_A, KC_B, KC_C], [KC_D, KC_E, KC_MO(1)]],
        [[KC_F, KC_G, KC_H], [KC_I, KC_J, KC_1]],
        [[KC_K, KC_L, KC_M], [KC_N, KC_O, KC_2]],
    ];

    let (pins, watchdog, delay) = init();

    let col: [DynPin; NUMOFCOL] = [pins.gpio28.into(), pins.gpio26.into(), pins.gpio17.into()];
    let row: [DynPin; NUMOFROW] = [pins.gpio16.into(), pins.gpio15.into()];

    matrix_scaning(col, row, KEYS, watchdog, delay);
}
