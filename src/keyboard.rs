#[cfg(feature="encoders")]
use cortex_m::delay::Delay;

use crate::keycode::{Keycodes, Modifers};
#[cfg(feature="encoders")]
use crate::push_input_report;
#[cfg(feature="encoders")]
use crate::hardware::{Dir, Encoder};
use crate::usb::Report;

pub struct Keyboard<const COLS: usize, const ROWS: usize, const LAYERS: usize> {
    current_state: [[bool; COLS]; ROWS],
    old_current_state: [[bool; COLS]; ROWS],
    locked_keys: [[(bool, usize); COLS]; ROWS],
    pub layer: usize,
    last_layer: usize,
    pub index: usize,
    // pub report: KeyboardReport,
    pub report: Report,
}

impl<const COLS: usize, const ROWS: usize, const LAYERS: usize> Keyboard<COLS, ROWS, LAYERS> {
    pub fn new() -> Self {
        Keyboard {
            current_state: [[false; COLS]; ROWS],
            old_current_state: [[false; COLS]; ROWS],
            locked_keys: [[(false, 0); COLS]; ROWS],
            layer: 0,
            last_layer: 0,
            index: 0,
            // report: KeyboardReport { modifier: 0x00, reserved: 0x00, leds: 0x00, keycodes: [0x00; 6] },
            report: Report::default(),
        }
    }

    // TODO push on every key press
    pub fn key_press(&mut self, keycode: Keycodes, col: usize, row: usize) {
        self.current_state[row][col] = true;

        if !self.locked_keys[row][col].0 {
            match keycode {
                Keycodes::KC_MO(x) => {
                    self.locked_keys[row][col] = (true, self.layer);
                    self.last_layer = self.layer;
                    self.layer = x;
                }
                Keycodes::KC_LEFT_CTRL => {
                    self.report.modifier |= Modifers::MOD_LCTRL as u8;
                }
                Keycodes::KC_LEFT_SHIFT => {
                    self.report.modifier |= Modifers::MOD_LSHIFT as u8;
                }
                Keycodes::KC_LEFT_ALT => {
                    self.report.modifier |= Modifers::MOD_LALT as u8;
                }
                Keycodes::KC_LEFT_GUI => {
                    self.report.modifier |= Modifers::MOD_LGUI as u8;
                }
                Keycodes::KC_RIGHT_CTRL => {
                    self.report.modifier |= Modifers::MOD_LCTRL as u8;
                }
                Keycodes::KC_RIGHT_SHIFT => {
                    self.report.modifier |= Modifers::MOD_RSHIFT as u8;
                }
                Keycodes::KC_RIGHT_ALT => {
                    self.report.modifier |= Modifers::MOD_RALT as u8;
                }
                Keycodes::KC_RIGHT_GUI => {
                    self.report.modifier |= Modifers::MOD_RGUI as u8;
                }
                _ if keycode.is_consumer() => {
                    if let Ok(keycode) = keycode.try_into() {
                        self.report.consumer_control = keycode;
                    }
                }
                _ => {
                    if let Ok(keycode) = keycode.try_into() {
                        self.add_key(keycode);
                    }
                }
            }
        }
    }

    fn add_key(&mut self, keycode: u8) {
        self.report.keycodes[self.index] = keycode;
        self.index += 1;
    }

    pub fn key_release(&mut self, keys: [[[Keycodes; COLS]; ROWS]; LAYERS], col: usize, row: usize) {
        if self.old_current_state[row][col] {
            match keys[self.locked_keys[row][col].1][row][col] {
                Keycodes::KC_LAYER(x) => self.layer = x,
                Keycodes::KC_MO(_) => { 
                    self.layer = self.last_layer;
                    self.locked_keys[row][col] = (false, self.layer);
                }
                _ => {}
            }
        }
        self.current_state[row][col] = false;
    }

    #[cfg(feature="encoders")]
    pub fn update_encoder(&mut self, encoder: &mut Encoder<LAYERS>, delay: &mut Delay) {
        encoder.update();
        match encoder.dir {
            Dir::Cw => {
                let keycode = encoder.actions_clock_wise[self.layer];
                if let Ok(keycode) = keycode.try_into() {
                    // self.report.keycodes[self.index] = keycode;
                    self.add_key(keycode);
                } else {
                    match keycode {
                        Keycodes::KC_LAYER(x) => self.layer = x as usize,
                        _ => {}
                    }
                }
                delay.delay_ms(30);
                push_input_report(self.report).ok().unwrap_or(0);
                // self.report.keycodes[self.index] = 0x00;
            }
            Dir::Cww => {
                let keycode = encoder.actions_counter_clock_wise[self.layer];
                if let Ok(keycode) = keycode.try_into() {
                    // self.report.keycodes[self.index] = keycode;
                    self.add_key(keycode);
                } else {
                    match keycode {
                        Keycodes::KC_LAYER(x) => self.layer = x as usize,
                        _ => {}
                    }
                }
                delay.delay_ms(30);
                push_input_report(self.report).ok().unwrap_or(0);
                // self.report.keycodes[self.index] = 0x00;
            }
            _ => {}
        }
    }

    pub fn update_state(&mut self) {
        self.old_current_state = self.current_state;
    }

    pub fn reset(&mut self) {
        self.index = 0;
        // self.report = KeyboardReport { modifier: 0x00, reserved: 0x00, leds: 0x00, keycodes: [0x00; 6] };
        self.report = Report::default();
    }
}
