use rp2040_hal::gpio::DynPin;
use rp2040_hal::usb::UsbBus;
use rp2040_hal::Timer;
use usb_device::class_prelude::UsbBusAllocator;

#[cfg(feature = "encoders")]
use crate::hardware::Encoder;
use crate::keycode::Keycode;

mod matrix;
mod state;
mod usb;

use matrix::Matrix;
use state::State;
use usb::Usb;

#[cfg(feature = "encoders")]
struct EncoderController<const NUM_OF_ENCODERS: usize> {
    encoders: [Encoder; NUM_OF_ENCODERS],
    encoders_state: [Keycode; NUM_OF_ENCODERS],
}

#[cfg(feature = "encoders")]
impl<'a, const NUM_OF_ENCODERS: usize> EncoderController<NUM_OF_ENCODERS> {
    fn new(encoders: [Encoder; NUM_OF_ENCODERS]) -> Self {
        Self {
            encoders,
            encoders_state: [Keycode::KC_A; NUM_OF_ENCODERS],
        }
    }

    // initializes the encoder controller
    fn initialize(&mut self) {
        self.encoders
            .iter_mut()
            .for_each(|encoder| encoder.initialize());
    }

    fn periodic(&mut self) {
        self.encoders
            .iter_mut()
            .for_each(|encoder| encoder.update());
    }

    fn actions(&self, layer: usize) -> [Keycode; NUM_OF_ENCODERS] {
        let mut actions = [Keycode::KC_NO; NUM_OF_ENCODERS];

        self.encoders
            .iter()
            .map(|encoder| encoder.action(layer))
            .enumerate()
            .for_each(|(index, action)| {
                actions[index] = action.unwrap_or(Keycode::KC_NO);
            });
        actions
    }
}

#[cfg(feature = "encoders")]
pub struct Keyboard<
    'a,
    const NUM_OF_COLS: usize,
    const NUM_OF_ROWS: usize,
    const NUM_OF_ENCODERS: usize,
> where
    [(); NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS]: Sized,
{
    state: State<'a, NUM_OF_COLS, NUM_OF_ROWS>,
    matrix: Matrix<'a, NUM_OF_COLS, NUM_OF_ROWS>,
    usb: Usb<'a>,
    encoder_controller: EncoderController<NUM_OF_ENCODERS>,
    buffer: [Keycode; NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS],
}

#[cfg(feature = "encoders")]
impl<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize, const NUM_OF_ENCODERS: usize>
    Keyboard<'a, NUM_OF_COLS, NUM_OF_ROWS, NUM_OF_ENCODERS>
where
    [(); NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS]: Sized,
{
    pub fn new(
        layout: &'a [&[&[Keycode]]],
        output_pins: &'a mut [DynPin],
        input_pins: &'a mut [DynPin],
        encoders: [Encoder; NUM_OF_ENCODERS],
        timer: &'a Timer,
        usb_bus: &'a UsbBusAllocator<UsbBus>,
    ) -> Self {
        Self {
            state: State::new(layout),
            matrix: Matrix::new(output_pins, input_pins, timer),
            usb: Usb::new(usb_bus, timer),
            encoder_controller: EncoderController::new(encoders),
            buffer: [Keycode::KC_NO; NUM_OF_COLS * NUM_OF_ROWS + NUM_OF_ENCODERS],
        }
    }

    // initialize the keyboard
    pub fn initialize(&mut self) {
        // initialize the matrix
        self.matrix.initialize();
        // initialize the usb controller
        self.usb.initialize();

        // initialize the encoder controller
        #[cfg(feature = "encoders")]
        self.encoder_controller.initialize();
    }

    // update the keyboard
    pub fn periodic(&mut self) {
        if self.matrix.scan(&mut self.state) {
            let flatten_state = self.matrix.state.flatten();
            let mut index = 0;

            flatten_state.iter().for_each(|keycode| {
                self.buffer[index] = *keycode;
                index += 1;
            });

            #[cfg(feature = "encoders")]
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

        #[cfg(feature = "encoders")]
        {
            self.encoder_controller.periodic();
            self.encoder_controller.encoders_state =
                self.encoder_controller.actions(self.state.layer());
        }
        self.usb.periodic();
    }
}


#[cfg(not(feature = "encoders"))]
pub struct Keyboard<
    'a,
    const NUM_OF_COLS: usize,
    const NUM_OF_ROWS: usize,
> where
    [(); NUM_OF_COLS * NUM_OF_ROWS]: Sized,
{
    state: State<'a, NUM_OF_COLS, NUM_OF_ROWS>,
    matrix: Matrix<'a, NUM_OF_COLS, NUM_OF_ROWS>,
    usb: Usb<'a>,
    #[cfg(not(feature = "encoders"))]
    buffer: [Keycode; NUM_OF_COLS * NUM_OF_ROWS],
}


#[cfg(not(feature = "encoders"))]
impl<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize>
    Keyboard<'a, NUM_OF_COLS, NUM_OF_ROWS>
where
    [(); NUM_OF_COLS * NUM_OF_ROWS]: Sized,
{
    pub fn new(
        layout: &'a [&[&[Keycode]]],
        output_pins: &'a mut [DynPin],
        input_pins: &'a mut [DynPin],
        timer: &'a Timer,
        usb_bus: &'a UsbBusAllocator<UsbBus>,
    ) -> Self {
        Self {
            state: State::new(layout),
            matrix: Matrix::new(output_pins, input_pins, timer),
            usb: Usb::new(usb_bus, timer),
            buffer: [Keycode::KC_NO; NUM_OF_COLS * NUM_OF_ROWS],
        }
    }

    // initialize the keyboard
    pub fn initialize(&mut self) {
        // initialize the matrix
        self.matrix.initialize();
        // initialize the usb controller
        self.usb.initialize();
    }

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
