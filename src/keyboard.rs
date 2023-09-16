#[cfg(feature = "encoders")]
use cortex_m::delay::Delay;
use usb_device::class_prelude::UsbBus;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig;
use usbd_human_interface_device::page;
use usbd_human_interface_device::usb_class::{UsbHidClass, UsbHidClassBuilder};

#[cfg(feature = "encoders")]
use crate::hardware::{Dir, Encoder};
use crate::key::Key;
use crate::keycode::Keycodes;

pub struct Keyboard<const COLS: usize, const ROWS: usize, const LAYERS: usize> {
    state: [[Option<Keycodes>; COLS]; ROWS],
    current_state: [[bool; COLS]; ROWS],
    old_current_state: [[bool; COLS]; ROWS],
    locked_keys: [[(bool, usize); COLS]; ROWS],
    pub layer: usize,
    last_layer: usize,
}

impl<const COLS: usize, const ROWS: usize, const LAYERS: usize> Keyboard<COLS, ROWS, LAYERS> {
    pub fn new() -> Self {
        Keyboard {
            state: [[None; COLS]; ROWS],
            current_state: [[false; COLS]; ROWS],
            old_current_state: [[false; COLS]; ROWS],
            locked_keys: [[(false, 0); COLS]; ROWS],
            layer: 0,
            last_layer: 0,
        }
    }

    // TODO push on every key press
    pub fn key_press(&mut self, key: Key) {
        if !key.encoder {
            self.current_state[key.row.unwrap_or(0)][key.col.unwrap_or(0)] = true;
        }

        if key.encoder || !self.locked_keys[key.row.unwrap_or(0)][key.col.unwrap_or(0)].0 {
            match key.keycode {
                Keycodes::KC_MO(x) => {
                    if !key.encoder {
                        self.locked_keys[key.row.unwrap_or(0)][key.col.unwrap_or(0)] =
                            (true, self.layer);
                        self.last_layer = self.layer;
                        self.layer = x;
                    }
                }
                Keycodes::KEYS_2(key_1, key_2) => {
                    let key = Key {
                        col: None,
                        row: None,
                        keycode: *key_1,
                        encoder: false,
                    };
                    self.key_press(key);
                    let key = Key {
                        col: None,
                        row: None,
                        keycode: *key_2,
                        encoder: false,
                    };
                    self.key_press(key);
                }
                Keycodes::KC_LAYER(x) => self.layer = x,
                _ if key.keycode.is_consumer() => {
                    // if let Ok(keycode) = key.keycode.try_into() {
                    // self.report.consumer_control = keycode;
                    // }
                }
                _ => {
                    // if let Ok(keycode) = key.keycode.try_into() {
                    self.add_key(key);
                    // }
                }
            }
        }
    }

    fn add_key(&mut self, key: Key) {
        self.state[key.row.unwrap()][key.col.unwrap()] = Some(key.keycode);
        // self.report.keycodes[self.index] = keycode;
        // self.index += 1;
    }

    pub fn key_release(
        &mut self,
        keys: [[[Keycodes; COLS]; ROWS]; LAYERS],
        col: usize,
        row: usize,
    ) {
        self.state[row][col] = None;
        // if self.old_current_state[row][col] {
        //     match keys[self.locked_keys[row][col].1][row][col] {
        //         Keycodes::KC_LAYER(x) => self.layer = x,
        //         Keycodes::KC_MO(_) => {
        //             self.layer = self.last_layer;
        //             self.locked_keys[row][col] = (false, self.layer);
        //         }
        //         _ => {}
        //     }
        // }
        // self.current_state[row][col] = false;
    }

    #[cfg(feature = "encoders")]
    pub fn update_encoder(&mut self, encoder: &mut Encoder<LAYERS>, delay: &mut Delay) {
        encoder.update();
        match encoder.dir {
            Dir::Cw => {
                let keycode = encoder.actions[self.layer][1];
                let key = Key {
                    col: None,
                    row: None,
                    keycode,
                    encoder: true,
                };
                self.key_press(key);
                // delay.delay_ms(30);
                // push_input_report(self.report).ok().unwrap_or(0);
            }
            Dir::Cww => {
                let keycode = encoder.actions[self.layer][0];
                let key = Key {
                    col: None,
                    row: None,
                    keycode,
                    encoder: true,
                };
                self.key_press(key);
                // delay.delay_ms(30);
                // push_input_report(self.report).ok().unwrap_or(0);
            }
            _ => {}
        }
    }

    pub fn state(&self) -> &[Option<Keycodes>] {
        self.state.flatten()
    }

    pub fn update_state(&mut self) {
        self.old_current_state = self.current_state;
    }

    // pub fn reset(&mut self) {
    //     self.index = 0;
    // }
}
