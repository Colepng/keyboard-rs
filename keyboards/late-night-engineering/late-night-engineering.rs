#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)] // allows the #[rustfmt::skip]

use keyboard_rs::hardware::Encoder;
use keyboard_rs::keycode::{Keycode, Keycode::*};
use keyboard_rs::{init, matrix_scaning, Board};

use panic_halt as _;
use rp2040_hal::gpio::PullUp;
use rp2040_hal::timer::CountDown;
use rp2040_hal::usb::UsbBus;
use rp2040_hal::{entry, Watchdog};
use rp_pico::hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, SioInput, SioOutput};

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 14;
    const NUMOFROW: usize = 5;
    const NUMOFLAYES: usize = 2;

    #[rustfmt::skip]
    const KEYS: &[&[&[Keycode]]] = &[
        &[
            &[KC_ESCAPE,     KC_1,       KC_2,       KC_3,       KC_4,       KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE], 
            &[KC_TAB,        KC_Q,       KC_W,       KC_E,       KC_R,       KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFT_BRACKET, KC_RIGHT_BRACKET,  KC_BACKSLASH], 
            &[KC_CAPS_LOCK,  KC_NO_KEY,  KC_A,       KC_S,       KC_D,       KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_QUOTE, KC_ENTER], 
            &[KC_LEFT_SHIFT, KC_NO_KEY,  KC_Z,       KC_X,       KC_C,       KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, KC_NO_KEY, KC_RIGHT_SHIFT], 
            &[KC_LEFT_CTRL,  KC_LEFT_GUI,KC_NO_KEY,  KC_LEFT_ALT,KC_NO_KEY,  KC_NO_KEY, KC_NO_KEY, KC_SPACE, KC_NO_KEY, KC_NO_KEY, KC_RIGHT_ALT, KC_MO(1), KC_APP, KC_RIGHT_CTRL],
        ],
        &[
            &[KC_GRAVE,     KC_F1,       KC_F2,      KC_F3,      KC_F4,      KC_F5, KC_F6, KC_F7, KC_F8, KC_F9, KC_F10, KC_F11, KC_F12, KC_DELETE_FORWARD],
            &[KC_TRANS,     KC_Q,        KC_UP_ARROW,KC_TRANS,   KC_TRANS,   KC_TRANS,  KC_TRANS,  KC_TRANS,  KC_TRANS,  KC_TRANS,  KC_TRANS,KC_TRANS,KC_TRANS, KC_TRANS], 
            &[KC_TRANS,     KC_NO_KEY,   KC_LEFT_ARROW,KC_DOWN_ARROW,KC_RIGHT_ARROW,KC_TRANS,  KC_TRANS,  KC_TRANS,  KC_TRANS,  KC_TRANS,   KC_TRANS,   KC_TRANS,KC_TRANS,KC_TRANS], 
            &[KC_MPLAY_PAUSE,KC_NO_KEY,  KC_TRANS,   KC_TRANS,   KC_TRANS,       KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_MNEXT], 
            &[KC_TRANS,     KC_TRANS,    KC_TRANS,   KC_TRANS,   KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_TRANS, KC_MPREV],
        ],
    ];

    let (pins, board, timer) = init();

    let (timer0, timer1) = Board::setup_timers(&timer, &timer);

    let col: &mut [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>] = &mut [
        pins.gpio26.into_push_pull_output().into_dyn_pin(),
        pins.gpio22.into_push_pull_output().into_dyn_pin(),
        pins.gpio16.into_push_pull_output().into_dyn_pin(),
        pins.gpio17.into_push_pull_output().into_dyn_pin(),
        pins.gpio18.into_push_pull_output().into_dyn_pin(),
        pins.gpio19.into_push_pull_output().into_dyn_pin(),
        pins.gpio20.into_push_pull_output().into_dyn_pin(),
        pins.gpio21.into_push_pull_output().into_dyn_pin(),
        pins.gpio10.into_push_pull_output().into_dyn_pin(),
        pins.gpio11.into_push_pull_output().into_dyn_pin(),
        pins.gpio12.into_push_pull_output().into_dyn_pin(),
        pins.gpio13.into_push_pull_output().into_dyn_pin(),
        pins.gpio14.into_push_pull_output().into_dyn_pin(),
        pins.gpio15.into_push_pull_output().into_dyn_pin(),
    ];
    let row: &mut [Pin<DynPinId, FunctionSio<SioInput>, PullDown>] = &mut [
        pins.gpio28.into_pull_down_input().into_dyn_pin(),
        pins.gpio5.into_pull_down_input().into_dyn_pin(),
        pins.gpio4.into_pull_down_input().into_dyn_pin(),
        pins.gpio3.into_pull_down_input().into_dyn_pin(),
        pins.gpio2.into_pull_down_input().into_dyn_pin(),
    ];

    let encoder1 = Encoder::new(
        pins.gpio9.into_pull_up_input().into_dyn_pin(),
        pins.gpio8.into_pull_up_input().into_dyn_pin(),
        #[rustfmt::skip]
        &[
            [KC_NO, KC_NO],
            [KC_NO, KC_NO]
        ],
    );

    let encoder2 = Encoder::new(
        pins.gpio7.into_pull_up_input().into_dyn_pin(),
        pins.gpio6.into_pull_up_input().into_dyn_pin(),
        #[rustfmt::skip]
        &[
            [KC_MINUS, KEYS_2(&KC_LEFT_SHIFT, &KC_EQUAL)],
            [KC_NO, KC_NO],
        ],
    );

    let encoder3 = Encoder::new(
        pins.gpio1.into_pull_up_input().into_dyn_pin(),
        pins.gpio0.into_pull_up_input().into_dyn_pin(),
        #[rustfmt::skip]
        &[
            [KC_VOLDOWN, KC_VOLUP],
            [KC_NO, KC_NO],
        ],
    );

    matrix_scaning::<
        NUMOFCOL,
        NUMOFROW,
        NUMOFLAYES,
        3,
        Pin<DynPinId, FunctionSio<SioInput>, PullUp>,
        Pin<DynPinId, FunctionSio<SioOutput>, PullDown>,
        Pin<DynPinId, FunctionSio<SioInput>, PullDown>,
        CountDown,
        Watchdog,
        UsbBus,
    >(
        board,
        col,
        row,
        KEYS,
        [encoder1, encoder2, encoder3],
        timer0,
        timer1,
    );
}
