use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    timer::CountDown,
};
use fugit::ExtU32;
use rp2040_hal::{gpio::DynPin, timer::CountDown as RPCountDown, Timer};

use crate::keycode::Keycode;

use super::State;

pub(super) struct Matrix<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize> {
    pub state: [[Keycode; NUM_OF_COLS]; NUM_OF_ROWS],
    output_pins: &'a mut [DynPin],
    input_pins: &'a mut [DynPin],
    timer: RPCountDown<'a>,
}

impl<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize> Matrix<'a, NUM_OF_COLS, NUM_OF_ROWS> {
    pub(super) fn new(
        output_pins: &'a mut [DynPin],
        input_pins: &'a mut [DynPin],
        timer: &'a Timer,
    ) -> Self {
        Self {
            state: [[Keycode::KC_NO; NUM_OF_COLS]; NUM_OF_ROWS],
            output_pins,
            input_pins,
            timer: timer.count_down(),
        }
    }

    pub(super) fn initialize(&mut self) {
        // initialize output_pins
        self.output_pins.iter_mut().for_each(|pin| {
            pin.into_readable_output();
        });

        // initialize input_pins
        self.input_pins.iter_mut().for_each(|pin| {
            pin.into_pull_down_input();
        });

        // initialize scan rate timer
        self.timer.start(10.millis());
    }

    // scans the matrix
    // returns if the matrix has changed
    pub(super) fn scan(&mut self, state: &State) -> bool {
        if !self.should_scan() {
            let mut has_changed = false;

            self.output_pins
                .iter_mut()
                .enumerate()
                .for_each(|(output_index, output_pin)| {
                    output_pin.set_high().unwrap();

                    self.input_pins
                        .iter_mut()
                        .enumerate()
                        .for_each(|(input_index, input_pin)| {
                            if input_pin.is_high().unwrap() {
                                if !has_changed {
                                    has_changed = true;
                                }

                                self.state[input_index][output_index] =
                                    state.get_key(input_index, output_index);
                            } else {
                                if !has_changed {
                                    has_changed = true;
                                }

                                self.state[input_index][output_index] = Keycode::KC_NO;
                            }
                        });

                    output_pin.set_low().unwrap();
                });

            has_changed
        } else {
            false
        }
    }

    // checks if the matrix should be scanned
    fn should_scan(&mut self) -> bool {
        self.timer.wait().is_ok()
    }
}
