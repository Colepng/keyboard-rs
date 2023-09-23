#![no_std]
#![no_main]

use keyboard_rs::keycode::{Keycode, Keycode::*};
use keyboard_rs::{init, matrix_scaning, Board};

use panic_halt as _;
use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, SioInput, SioOutput};
use rp2040_hal::timer::CountDown;
use rp2040_hal::Watchdog;
use rp2040_hal::usb::UsbBus;
use rp_pico::entry;

type Input = Pin<DynPinId, FunctionSio<SioInput>, PullDown>;
type Output = Pin<DynPinId, FunctionSio<SioOutput>, PullDown>;

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

    let (pins, board, timer) = init();

    let (timer0, timer1) = Board::setup_timers(&timer, &timer);

    let col: &mut [Output] = &mut [
        pins.gpio28.into_push_pull_output().into_dyn_pin(),
        pins.gpio26.into_push_pull_output().into_dyn_pin(),
        pins.gpio17.into_push_pull_output().into_dyn_pin(),
    ];
    let row: &mut [Input] = &mut [
        pins.gpio16.into_pull_down_input().into_dyn_pin(),
        pins.gpio15.into_pull_down_input().into_dyn_pin(),
    ];

    matrix_scaning::<NUMOFCOL, NUMOFROW, NUMOFLAYES, Output, Input, CountDown, Watchdog, UsbBus>(
        board, col, row, KEYS, timer0, timer1,
    );
}
