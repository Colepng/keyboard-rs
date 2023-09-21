use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    timer::CountDown,
};
use fugit::ExtU32;
use rp2040_hal::{timer::CountDown as RPCountDown, Timer};

use crate::keycode::Keycode;

use super::State;

pub(super) struct Matrix<
    'a,
    const NUM_OF_COLS: usize,
    const NUM_OF_ROWS: usize,
    Output: OutputPin,
    Input: InputPin,
> {
    pub state: [[Keycode; NUM_OF_COLS]; NUM_OF_ROWS],
    output_pins: &'a mut [Output],
    input_pins: &'a mut [Input],
    timer: RPCountDown<'a>,
}

impl<
        'a,
        const NUM_OF_COLS: usize,
        const NUM_OF_ROWS: usize,
        Output: OutputPin,
        Input: InputPin,
    > Matrix<'a, NUM_OF_COLS, NUM_OF_ROWS, Output, Input>
{
    pub(super) fn new(
        output_pins: &'a mut [Output],
        input_pins: &'a mut [Input],
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
        // initialize scan rate timer
        self.timer.start(10.millis());
    }

    // scans the matrix
    // returns if the matrix has changed
    pub(super) fn scan(&mut self, state: &mut State<NUM_OF_COLS, NUM_OF_ROWS>) -> bool {
        if !self.should_scan() {
            let mut has_changed = false;

            self.output_pins
                .iter_mut()
                .enumerate()
                .for_each(|(output_index, output_pin)| {
                    if output_pin.set_high().is_err() {
                        panic!("");
                    }

                    self.input_pins
                        .iter_mut()
                        .enumerate()
                        .for_each(|(input_index, input_pin)| {
                            let result: bool;
                            if let Ok(result1) = input_pin.is_high() {
                                result = result1;
                            } else {
                                panic!()
                            }
                            if result {
                                if !has_changed {
                                    has_changed = true;
                                }

                                let key = state.get_key(input_index, output_index);
                                if self.state[input_index][output_index] != key {
                                    state.on_press(key, input_index, output_index);
                                    self.state[input_index][output_index] = key;
                                }
                            } else {
                                if !has_changed {
                                    has_changed = true;
                                }

                                if self.state[input_index][output_index] != Keycode::KC_NO {
                                    state.on_release(
                                        self.state[input_index][output_index],
                                        input_index,
                                        output_index,
                                    );
                                    self.state[input_index][output_index] = Keycode::KC_NO;
                                }
                            }
                        });

                    if output_pin.set_low().is_err() {
                        panic!("");
                    }
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
