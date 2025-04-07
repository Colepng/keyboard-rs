use core::prelude::rust_2024::*;
use either::Either;
use embedded_hal::digital::v2::InputPin;

pub struct Encoder<PinA: InputPin, PinB: InputPin> {
    clk: PinA,
    dt: PinB,
    state: u8,
    pulses: i8,
    dir: Dir,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    Cw,
    Cww,
    Same,
}

impl<PinA: InputPin, PinB: InputPin> Encoder<PinA, PinB> {
    const LOOKUP_TABLE: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

    pub const fn new(pin_a: PinA, pin_b: PinB) -> Self {
        Self {
            clk: pin_a,
            dt: pin_b,
            state: 0,
            pulses: 0,
            dir: Dir::Same,
        }
    }

    /// Returns the update of this [`Encoder<PinA, PinB>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if either input pins return errors.
    pub fn update(&mut self) -> Result<(), Either<PinA::Error, PinB::Error>> {
        #[rustfmt::skip]
        let clk: u8 = u8::from(self.clk.is_high().map_err(Either::Left)?);
        let dt: u8 = u8::from(self.dt.is_high().map_err(Either::Right)?);

        let new_state: u8 = clk << 1 | dt;

        #[allow(clippy::if_not_else)]
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
        Ok(())
    }

    pub const fn direction(&self) -> Dir {
        self.dir
    }
}

// mod test {
//     use embedded_hal::{digital::v2::InputPin, digital::v2::{ErrorType, Error}};
//
//     use crate::encoder::Dir;
//
//     use super::Encoder;
//
//     struct FakeInputPin {
//         state: bool
//     }
//
//     impl FakeInputPin {
//         fn new(state: bool) -> Self {
//             Self {
//                 state,
//             }
//         }
//
//         fn set_state(&mut self, state: bool) {
//             self.state = state;
//         }
//     }
//
//     #[derive(Debug)]
//     struct CantError;
//
//     impl Error for CantError {
//         fn kind(&self) -> embedded_hal::digital::ErrorKind {
//             embedded_hal::digital::ErrorKind::Other
//         }
//     }
//
//     impl ErrorType for FakeInputPin {
//         type Error = CantError;
//     }
//
//     impl InputPin for FakeInputPin {
//         fn is_high(&mut self) -> Result<bool, Self::Error> {
//             Ok(self.state == true)
//         }
//
//         fn is_low(&mut self) -> Result<bool, Self::Error> {
//             Ok(self.state == false)
//         }
//     }
//
//     fn test() {
//         let pin_a = FakeInputPin::new(false);
//         let pin_b = FakeInputPin::new(false);
//
//         let encoder = Encoder::new(pin_a, pin_b);
//
//         assert_eq!(encoder.direction(), Dir::Same);
//     }
// }
