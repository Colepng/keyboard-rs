use rp_pico::hal::gpio::bank0;
use rp_pico::hal::gpio::Input;
use rp_pico::hal::gpio::Output;
use rp_pico::hal::gpio::Pin;
use rp_pico::hal::gpio::PullDown;
use rp_pico::hal::gpio::Readable;

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

#[derive(PartialEq)]
pub enum PinMode {
    High,
    Low,
}

#[allow(dead_code)]
pub enum OutputPins {
    GP28(Pin<bank0::Gpio28, Output<Readable>>),
    GP27(Pin<bank0::Gpio27, Output<Readable>>),
    GP26(Pin<bank0::Gpio26, Output<Readable>>),
    GP25(Pin<bank0::Gpio25, Output<Readable>>),
    GP24(Pin<bank0::Gpio24, Output<Readable>>),
    GP23(Pin<bank0::Gpio23, Output<Readable>>),
    GP22(Pin<bank0::Gpio22, Output<Readable>>),
    GP21(Pin<bank0::Gpio21, Output<Readable>>),
    GP20(Pin<bank0::Gpio20, Output<Readable>>),
    GP19(Pin<bank0::Gpio19, Output<Readable>>),
    GP18(Pin<bank0::Gpio18, Output<Readable>>),
    GP17(Pin<bank0::Gpio17, Output<Readable>>),
    GP16(Pin<bank0::Gpio16, Output<Readable>>),
    GP15(Pin<bank0::Gpio15, Output<Readable>>),
    GP14(Pin<bank0::Gpio14, Output<Readable>>),
    GP13(Pin<bank0::Gpio13, Output<Readable>>),
    GP12(Pin<bank0::Gpio12, Output<Readable>>),
    GP11(Pin<bank0::Gpio11, Output<Readable>>),
    GP10(Pin<bank0::Gpio10, Output<Readable>>),
    GP9(Pin<bank0::Gpio9, Output<Readable>>),
    GP8(Pin<bank0::Gpio8, Output<Readable>>),
    GP7(Pin<bank0::Gpio7, Output<Readable>>),
    GP6(Pin<bank0::Gpio6, Output<Readable>>),
    GP5(Pin<bank0::Gpio5, Output<Readable>>),
    GP4(Pin<bank0::Gpio4, Output<Readable>>),
    GP3(Pin<bank0::Gpio3, Output<Readable>>),
    GP2(Pin<bank0::Gpio2, Output<Readable>>),
    GP1(Pin<bank0::Gpio1, Output<Readable>>),
    GP0(Pin<bank0::Gpio0, Output<Readable>>),
}

#[allow(dead_code)]
pub enum InputPins {
    GP28(Pin<bank0::Gpio28, Input<PullDown>>),
    GP27(Pin<bank0::Gpio27, Input<PullDown>>),
    GP26(Pin<bank0::Gpio26, Input<PullDown>>),
    GP25(Pin<bank0::Gpio25, Input<PullDown>>),
    GP24(Pin<bank0::Gpio24, Input<PullDown>>),
    GP23(Pin<bank0::Gpio23, Input<PullDown>>),
    GP22(Pin<bank0::Gpio22, Input<PullDown>>),
    GP21(Pin<bank0::Gpio21, Input<PullDown>>),
    GP20(Pin<bank0::Gpio20, Input<PullDown>>),
    GP19(Pin<bank0::Gpio19, Input<PullDown>>),
    GP18(Pin<bank0::Gpio18, Input<PullDown>>),
    GP17(Pin<bank0::Gpio17, Input<PullDown>>),
    GP16(Pin<bank0::Gpio16, Input<PullDown>>),
    GP15(Pin<bank0::Gpio15, Input<PullDown>>),
    GP14(Pin<bank0::Gpio14, Input<PullDown>>),
    GP13(Pin<bank0::Gpio13, Input<PullDown>>),
    GP12(Pin<bank0::Gpio12, Input<PullDown>>),
    GP11(Pin<bank0::Gpio11, Input<PullDown>>),
    GP10(Pin<bank0::Gpio10, Input<PullDown>>),
    GP9(Pin<bank0::Gpio9, Input<PullDown>>),
    GP8(Pin<bank0::Gpio8, Input<PullDown>>),
    GP7(Pin<bank0::Gpio7, Input<PullDown>>),
    GP6(Pin<bank0::Gpio6, Input<PullDown>>),
    GP5(Pin<bank0::Gpio5, Input<PullDown>>),
    GP4(Pin<bank0::Gpio4, Input<PullDown>>),
    GP3(Pin<bank0::Gpio3, Input<PullDown>>),
    GP2(Pin<bank0::Gpio2, Input<PullDown>>),
    GP1(Pin<bank0::Gpio1, Input<PullDown>>),
    GP0(Pin<bank0::Gpio0, Input<PullDown>>),
}

impl OutputPins {
    pub fn set_output_pin_mode(&mut self, mode: PinMode) {
        match self {
            Self::GP28(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP27(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP26(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP25(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP24(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP23(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP22(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP21(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP20(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP19(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP18(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP17(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP16(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP15(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP14(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP13(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP12(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP11(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP10(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP9(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP8(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP7(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP6(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP5(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP4(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP3(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP2(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP1(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
            Self::GP0(pin) => {if mode == PinMode::High {pin.set_high().unwrap()} else {pin.set_low().unwrap()}}
        }
    }
}

impl InputPins {
    pub fn is_high(&self) -> bool {
        match self {
            Self::GP28(pin) => {return pin.is_high().unwrap();}
            Self::GP27(pin) => {return pin.is_high().unwrap();}
            Self::GP26(pin) => {return pin.is_high().unwrap();}
            Self::GP25(pin) => {return pin.is_high().unwrap();}
            Self::GP24(pin) => {return pin.is_high().unwrap();}
            Self::GP23(pin) => {return pin.is_high().unwrap();}
            Self::GP22(pin) => {return pin.is_high().unwrap();}
            Self::GP21(pin) => {return pin.is_high().unwrap();}
            Self::GP20(pin) => {return pin.is_high().unwrap();}
            Self::GP19(pin) => {return pin.is_high().unwrap();}
            Self::GP18(pin) => {return pin.is_high().unwrap();}
            Self::GP17(pin) => {return pin.is_high().unwrap();}
            Self::GP16(pin) => {return pin.is_high().unwrap();}
            Self::GP15(pin) => {return pin.is_high().unwrap();}
            Self::GP14(pin) => {return pin.is_high().unwrap();}
            Self::GP13(pin) => {return pin.is_high().unwrap();}
            Self::GP12(pin) => {return pin.is_high().unwrap();}
            Self::GP11(pin) => {return pin.is_high().unwrap();}
            Self::GP10(pin) => {return pin.is_high().unwrap();}
            Self::GP9(pin) => {return pin.is_high().unwrap();}
            Self::GP8(pin) => {return pin.is_high().unwrap();}
            Self::GP7(pin) => {return pin.is_high().unwrap();}
            Self::GP6(pin) => {return pin.is_high().unwrap();}
            Self::GP5(pin) => {return pin.is_high().unwrap();}
            Self::GP4(pin) => {return pin.is_high().unwrap();}
            Self::GP3(pin) => {return pin.is_high().unwrap();}
            Self::GP2(pin) => {return pin.is_high().unwrap();}
            Self::GP1(pin) => {return pin.is_high().unwrap();}
            Self::GP0(pin) => {return pin.is_high().unwrap();}
        }
    }
}
