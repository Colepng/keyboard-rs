use embedded_hal::digital::v2::InputPin;
use rp2040_hal::gpio::DynPin;

use crate::keycode::Keycode;

pub struct Encoder {
    channel_a: DynPin,
    channel_b: DynPin,
    pub(super) actions: &'static [[Keycode; 2]],
    state: u8,
    pulses: i8,
    dir: Dir,
}

pub enum Dir {
    Cw,
    Cww,
    Same,
}

impl Encoder {
    pub const LOOKUP_TABLE: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

    pub fn new(channel_a: DynPin, channel_b: DynPin, actions: &'static [[Keycode; 2]]) -> Self {
        Encoder {
            channel_a,
            channel_b,
            actions,
            state: 0,
            pulses: 0,
            dir: Dir::Same,
        }
    }

    pub(super) fn update(&mut self) {
        #[rustfmt::skip]
        let new_state: u8 = (self.channel_a.is_high().unwrap() as u8) << 1 | (self.channel_b.is_high().unwrap() as u8);
        if self.state & 0b0011 != new_state {
            self.state <<= 2;
            self.state |= new_state;

            self.pulses += Self::LOOKUP_TABLE[self.state as usize & 0b1111];
            if self.pulses == 4 {
                self.dir = Dir::Cw;
            } else if self.pulses == -4 {
                self.dir = Dir::Cww;
            } else {
                self.dir = Dir::Same;
            }
            self.pulses %= 4;
        } else {
            self.dir = Dir::Same;
        }
    }

    pub(super) fn initialize(&mut self) {
        self.channel_a.into_pull_up_input();
        self.channel_b.into_pull_up_input();
    }

    pub(super) fn action(&self, layer: usize) -> Option<Keycode> {
        match self.dir {
            Dir::Cw => Some(self.actions[layer][1]),
            Dir::Cww => Some(self.actions[layer][0]),
            _ => None,
        }
    }
}
