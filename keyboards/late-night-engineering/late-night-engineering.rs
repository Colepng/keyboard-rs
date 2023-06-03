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
    const NUMOFCOL: usize = 14;
    const NUMOFROW: usize = 5;
    const NUMOFLAYES: usize = 1;

    const KEYS: [[[Keycodes; NUMOFCOL]; NUMOFROW]; NUMOFLAYES] = [
        [
            [KC_ESCAPE,     KC_1,       KC_2,       KC_3,       KC_4,       KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE], 
            [KC_TAB, KC_Q,  KC_W,       KC_E,       KC_R,       KC_T,       KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFT_BRACKET, KC_RIGHT_BRACKET, KC_SLASH], 
            [KC_CAPS_LOCK,  KC_NO_KEY,  KC_A,       KC_S,       KC_D,       KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_QUOTE, KC_ENTER], 
            [KC_LEFT_SHIFT, KC_NO_KEY,  KC_Z,       KC_X,       KC_C,       KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, KC_NO_KEY, KC_RIGHT_SHIFT], 
            [KC_LEFT_CTRL,  KC_LEFT_GUI,KC_NO_KEY,  KC_LEFT_ALT,KC_NO_KEY, KC_NO_KEY, KC_NO_KEY, KC_SPACE, KC_NO_KEY, KC_NO_KEY, KC_RIGHT_ALT, KC_RIGHT_GUI, KC_APP, KC_RIGHT_CTRL],
        ],
    ];

    let (pins, watchdog, delay) = init();

    let col: [DynPin; NUMOFCOL] = [pins.gpio26.into(), pins.gpio22.into(), pins.gpio16.into(), pins.gpio17.into(),pins.gpio18.into(),pins.gpio19.into(),pins.gpio20.into(),pins.gpio21.into(),pins.gpio10.into(),pins.gpio11.into(),pins.gpio12.into(),pins.gpio13.into(), pins.gpio14.into(), pins.gpio15.into()];
    let row: [DynPin; NUMOFROW] = [pins.gpio28.into(), pins.gpio5.into(), pins.gpio4.into(), pins.gpio3.into(), pins.gpio2.into()];

    let config: Config = Config { encoder: true };

    let encoder1 = Encoder::new(
        pins.gpio9.into(),
        pins.gpio8.into(),
        [KC_VOLDOWN],
        [KC_VOLUP],
    );

    let encoder2 = Encoder::new(
        pins.gpio7.into(),
        pins.gpio6.into(),
        [KC_VOLDOWN],
        [KC_VOLUP],
    );

    let encoder3 = Encoder::new(
        pins.gpio1.into(),
        pins.gpio0.into(),
        [KC_VOLDOWN],
        [KC_VOLUP],
    );

    matrix_scaning(col, row, KEYS, Some(encoder1), config, watchdog, delay);
}
