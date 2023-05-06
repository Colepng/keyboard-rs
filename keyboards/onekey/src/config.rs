use crate::pins::{OutputPins, InputPins};

pub enum DiodeDirection{
    Col2Row,
    Row2Col,
}

pub const DIODE_DIRECTION: bool = false;

pub const KEYS: [[u8; 3]; 2] = [
    [0x04, 0x05, 0x06], 
    [0x07, 0x08, 0x09],
];

pub struct Matrix {
    pub cols: [OutputPins; 3],
    pub rows: [InputPins; 2],
}

impl Matrix {
    pub fn new(pins: rp_pico::Pins) -> Self {
        Matrix { 
            cols: [ 
                OutputPins::GP28(pins.gpio28.into_readable_output()),
                OutputPins::GP26(pins.gpio26.into_readable_output()),
                OutputPins::GP17(pins.gpio17.into_readable_output()),
            ],
            rows: [
                InputPins::GP16(pins.gpio16.into_pull_down_input()),
                InputPins::GP15(pins.gpio15.into_pull_down_input()),
            ],
        }
    }
}

