use either::Either;
use embedded_hal::digital::v2::InputPin;

pub struct Encoder<PinA, PinB> {
    clk: PinA,
    dt: PinB,
    state: u8,
    pulses: i8,
    dir: Dir,
}

pub enum Error<Error1, Error2> {
    Error1(Error1),
    Error2(Error2),
}

#[derive(Clone, Copy)]
pub enum Dir {
    Cw,
    Cww,
    Same,
}

impl<PinA: InputPin, PinB: InputPin> Encoder<PinA, PinB> {
    const LOOKUP_TABLE: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

    pub fn new(pin_a: PinA, pin_b: PinB) -> Self {
        Encoder {
            clk: pin_a,
            dt: pin_b,
            state: 0,
            pulses: 0,
            dir: Dir::Same,
        }
    }

    pub fn update(&mut self) -> Result<(), Either<PinA::Error, PinB::Error>> {
        #[rustfmt::skip]
        let clk: u8 = self.clk.is_high().map_err(Either::Left)? as u8;
        let dt: u8 = self.dt.is_high().map_err(Either::Right)? as u8;

        let new_state: u8 = clk << 1 | dt;
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

    pub fn direction(&self) -> Dir {
        self.dir
    }
}
