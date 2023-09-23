use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::timer::CountDown;
use usb_device::class_prelude::{UsbBus as UsbBusTrait, UsbBusAllocator};

#[cfg(feature = "encoders")]
use crate::hardware::encoder::Encoder;
use crate::keycode::Keycode;

#[cfg(feature = "encoders")]
mod encoder_controller;
mod matrix;
mod state;
mod usb;

#[cfg(feature = "encoders")]
use encoder_controller::EncoderController;
use matrix::Matrix;
use state::State;
use usb::Usb;

#[cfg(feature = "encoders")]
pub struct Keyboard<
    'a,
    const NUM_OF_COLS: usize,
    const NUM_OF_ROWS: usize,
    const NUM_OF_ENCODERS: usize,
    EncoderPin: InputPin,
    Output: OutputPin,
    Input: InputPin,
    Timer: CountDown,
    UsbBus: UsbBusTrait,
> where
    [(); NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS]: Sized,
{
    state: State<'a, NUM_OF_COLS, NUM_OF_ROWS>,
    matrix: Matrix<'a, NUM_OF_COLS, NUM_OF_ROWS, Output, Input, Timer>,
    usb: Usb<'a, Timer, UsbBus>,
    encoder_controller: EncoderController<NUM_OF_ENCODERS, EncoderPin>,
    buffer: [Keycode; NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS],
}

#[cfg(feature = "encoders")]
impl<
        'a,
        const NUM_OF_COLS: usize,
        const NUM_OF_ROWS: usize,
        const NUM_OF_ENCODERS: usize,
        EncoderPin: InputPin,
        Output: OutputPin,
        Input: InputPin,
        Timer: CountDown,
        UsbBus: UsbBusTrait,
    >
    Keyboard<
        'a,
        NUM_OF_COLS,
        NUM_OF_ROWS,
        NUM_OF_ENCODERS,
        EncoderPin,
        Output,
        Input,
        Timer,
        UsbBus,
    >
where
    [(); NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS]: Sized,
{
    pub fn new(
        layout: &'a [&[&[Keycode]]],
        output_pins: &'a mut [Output],
        input_pins: &'a mut [Input],
        encoders: [Encoder<EncoderPin>; NUM_OF_ENCODERS],
        timer0: &'a mut Timer,
        timer1: &'a mut Timer,
        usb_bus: &'a UsbBusAllocator<UsbBus>,
    ) -> Self {
        Self {
            state: State::new(layout),
            matrix: Matrix::new(output_pins, input_pins, timer0),
            usb: Usb::new(usb_bus, timer1),
            encoder_controller: EncoderController::new(encoders),
            buffer: [Keycode::KC_NO; NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS],
        }
    }

    // initialize the keyboard
    // pub fn initialize(&mut self) {
    //     self.matrix = Some(Matrix::new(output_pins, input_pins, &timers[0]));
    // }

    // update the keyboard
    pub fn periodic(&mut self) {
        if self.matrix.scan(&mut self.state) {
            let flatten_state = self.matrix.state.flatten();
            let mut index = 0;

            for keycode in flatten_state {
                self.buffer[index] = *keycode;
                index += 1;
            }

            self.encoder_controller
                .encoders_state
                .iter()
                .for_each(|keycode| {
                    self.buffer[index] = *keycode;
                    index += 1;
                });

            self.usb.write_keyboard_report(&self.buffer);
            self.usb.write_consumer_report(&self.buffer);
        }

        self.encoder_controller.periodic();
        self.encoder_controller.encoders_state =
            self.encoder_controller.actions(self.state.layer());

        self.usb.periodic();
    }
}

#[cfg(not(feature = "encoders"))]
pub struct Keyboard<
    'a,
    const NUM_OF_COLS: usize,
    const NUM_OF_ROWS: usize,
    Output: OutputPin,
    Input: InputPin,
    Timer: CountDown,
    UsbBus: UsbBusTrait,
> where
    [(); NUM_OF_COLS * NUM_OF_ROWS]: Sized,
{
    state: State<'a, NUM_OF_COLS, NUM_OF_ROWS>,
    matrix: Matrix<'a, NUM_OF_COLS, NUM_OF_ROWS, Output, Input, Timer>,
    usb: Usb<'a, Timer, UsbBus>,
    buffer: [Keycode; NUM_OF_COLS * NUM_OF_ROWS],
}

#[cfg(not(feature = "encoders"))]
impl<
        'a,
        const NUM_OF_COLS: usize,
        const NUM_OF_ROWS: usize,
        Output: OutputPin,
        Input: InputPin,
        Timer: CountDown,
        UsbBus: UsbBusTrait,
    > Keyboard<'a, NUM_OF_COLS, NUM_OF_ROWS, Output, Input, Timer, UsbBus>
where
    [(); NUM_OF_COLS * NUM_OF_ROWS]: Sized,
{
    pub fn new(
        layout: &'a [&[&[Keycode]]],
        output_pins: &'a mut [Output],
        input_pins: &'a mut [Input],
        timer0: &'a mut Timer,
        timer1: &'a mut Timer,
        usb_bus: &'a UsbBusAllocator<UsbBus>,
    ) -> Self {
        Self {
            state: State::new(layout),
            matrix: Matrix::new(output_pins, input_pins, timer0),
            usb: Usb::new(usb_bus, timer1),
            buffer: [Keycode::KC_NO; NUM_OF_COLS * NUM_OF_ROWS],
        }
    }

    // initialize the keyboard
    // pub fn initialize(&mut self) {
    //     // initialize the matrix
    //     self.matrix.initialize();
    //     // initialize the usb controller
    //     self.usb.initialize();
    // }

    // update the keyboard
    pub fn periodic(&mut self) {
        if self.matrix.scan(&mut self.state) {
            let flatten_state = self.matrix.state.flatten();
            let mut index = 0;

            flatten_state.iter().for_each(|keycode| {
                self.buffer[index] = *keycode;
                index += 1;
            });

            self.usb.write_keyboard_report(&self.buffer);
            self.usb.write_consumer_report(&self.buffer);
        }

        self.usb.periodic();
    }
}
