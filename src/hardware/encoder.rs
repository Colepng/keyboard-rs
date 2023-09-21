use embedded_hal::digital::v2::InputPin;
use hardware::encoder::{self, Dir};

use crate::keycode::Keycode;

pub struct Encoder<Input: InputPin> {
    encoder: encoder::Encoder<Input, Input>,
    pub(super) actions: &'static [[Keycode; 2]],
}

impl<Input: InputPin> Encoder<Input> {
    pub fn new(pin_a: Input, pin_b: Input, actions: &'static [[Keycode; 2]]) -> Self {
        Self {
            encoder: encoder::Encoder::new(pin_a, pin_b),
            actions,
        }
    }

    pub(crate) fn action(&self, layer: usize) -> Keycode {
        match self.encoder.direction() {
            Dir::Cw => self.actions[layer][1],
            Dir::Cww => self.actions[layer][0],
            _ => Keycode::KC_NO,
        }
    }

    pub(crate) fn update(&mut self) {
        let _ = self.encoder.update();
    }
}
