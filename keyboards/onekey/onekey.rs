#![no_std]
#![no_main]

use keyboard_rs::hardware::Encoder;
use keyboard_rs::keycode::{Keycodes, Keycodes::*};
use keyboard_rs::{init, matrix_scaning};

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 1;
    const NUMOFROW: usize = 1;
    const NUMOFLAYES: usize = 2;

    #[rustfmt::skip]
    const KEYS: [[[Keycodes; NUMOFCOL]; NUMOFROW]; NUMOFLAYES] = [
        [
            [KC_LAYER(1)]
        ], [
            [KC_LAYER(0)]
        ]
    ];

    let (pins, watchdog, delay) = init();

    let col: [DynPin; NUMOFCOL] = [pins.gpio27.into()];
    let row: [DynPin; NUMOFROW] = [pins.gpio18.into()];

    let encoder = Encoder::new(
        pins.gpio22.into(),
        pins.gpio21.into(),
        #[rustfmt::skip]
        [
            [KC_VOLUP, KC_B],
            [KC_VOLDOWN, KC_A],
        ],
    );

    matrix_scaning(col, row, KEYS, [encoder], watchdog, delay);
}
