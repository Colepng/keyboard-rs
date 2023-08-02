use embedded_hal::digital::v2::InputPin;
use rp2040_hal::gpio::DynPin;

use crate::keycode::Keycodes;

pub struct Encoder<const LAYERS: usize> {
    pub channel_a: DynPin,
    pub channel_b: DynPin,
    pub actions: [[Keycodes; 2]; LAYERS],
    state: u8,
    pulses: i8,
    pub dir: Dir,
}

pub enum Dir {
    Cw,
    Cww,
    Same,
}

impl<const LAYERS: usize> Encoder<LAYERS> {
    pub const LOOKUP_TABLE: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

    pub fn new(channel_a: DynPin, channel_b: DynPin, actions: [[Keycodes; 2]; LAYERS]) -> Self {
        Encoder {
            channel_a,
            channel_b,
            actions,
            state: 0,
            pulses: 0,
            dir: Dir::Same,
        }
    }

    pub fn update(&mut self) {
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
}
