#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)] // enables skipping rustfmt

use keyboard_rs::hardware::Encoder;
use keyboard_rs::keycode::{Keycode, Keycode::*};
use keyboard_rs::{init, matrix_scaning};
use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, PullUp, SioInput, SioOutput};

use panic_halt as _;
use rp_pico::entry;

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

    let col: &mut [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>] =
        &mut [pins.gpio27.into_push_pull_output().into_dyn_pin()];
    let row: &mut [Pin<DynPinId, FunctionSio<SioInput>, PullDown>] =
        &mut [pins.gpio18.into_pull_down_input().into_dyn_pin()];

    let encoder = Encoder::new(
        pins.gpio22.into_pull_up_input().into_dyn_pin(),
        pins.gpio21.into_pull_up_input().into_dyn_pin(),
        #[rustfmt::skip]
        &[
            [KC_VOLDOWN, KC_VOLUP],
            [KC_B, KC_A],
        ],
    );

    matrix_scaning::<
        NUMOFCOL,
        NUMOFROW,
        NUMOFLAYES,
        NUMOFENCODERS,
        Pin<DynPinId, FunctionSio<SioInput>, PullUp>,
        Pin<DynPinId, FunctionSio<SioOutput>, PullDown>,
        Pin<DynPinId, FunctionSio<SioInput>, PullDown>,
    >(board, col, row, KEYS, [encoder]);
}
