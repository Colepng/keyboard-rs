#![no_std]
#![no_main]

use keyboard_rs::keycode::{Keycode, Keycode::*};
use keyboard_rs::{init, matrix_scaning};

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 3;
    const NUMOFROW: usize = 2;
    const NUMOFLAYES: usize = 3;

    #[rustfmt::skip]
    const KEYS: &[&[&[Keycode]]] = &[
        &[
            &[KC_A, KC_B, KC_C], 
            &[KC_D, KC_E, KC_MO(1)]],
        &[
            &[KC_F, KC_G, KC_H], 
            &[KC_I, KC_J, KC_1]],
        &[
            &[KC_K, KC_L, KC_M], 
            &[KC_N, KC_O, KC_2]],
    ];

    let (pins, board) = init();

    let col: &mut [DynPin] = &mut [pins.gpio28.into(), pins.gpio26.into(), pins.gpio17.into()];
    let row: &mut [DynPin] = &mut [pins.gpio16.into(), pins.gpio15.into()];

    matrix_scaning::<NUMOFCOL, NUMOFROW, NUMOFLAYES>(board, col, row, KEYS);
}
