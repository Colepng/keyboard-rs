#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)] // enables skipping rustfmt

use keyboard_rs::hardware::Encoder;
use keyboard_rs::keycode::{Keycode, Keycode::*};
use keyboard_rs::{init, matrix_scaning};

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 1;
    const NUMOFROW: usize = 1;
    const NUMOFLAYES: usize = 2;
    const NUMOFENCODERS: usize = 1;

    #[rustfmt::skip]
    const KEYS: &[&[&[Keycode]]] = &[
        &[
            &[KC_LAYER(1)]
        ],
        &[
            &[KC_LAYER(0)]
        ]
    ];

    let (pins, board) = init();

    let col: &mut [DynPin] = &mut [pins.gpio27.into()];
    let row: &mut [DynPin] = &mut [pins.gpio18.into()];

    let encoder = Encoder::new(
        pins.gpio22.into(),
        pins.gpio21.into(),
        #[rustfmt::skip]
        &[
            [KC_VOLDOWN, KC_VOLUP],
            [KC_B, KC_A],
        ],
    );

    matrix_scaning::<NUMOFCOL, NUMOFROW, NUMOFLAYES, NUMOFENCODERS>(
        board,
        col,
        row,
        KEYS,
        [encoder],
    );
}
