use crate::hardware::Encoder;
use crate::keycode::Keycode;

#[cfg(feature = "encoders")]
pub(super) struct EncoderController<const NUM_OF_ENCODERS: usize> {
    encoders: [Encoder; NUM_OF_ENCODERS],
    pub encoders_state: [Keycode; NUM_OF_ENCODERS],
}

#[cfg(feature = "encoders")]
impl<'a, const NUM_OF_ENCODERS: usize> EncoderController<NUM_OF_ENCODERS> {
    pub(super) fn new(encoders: [Encoder; NUM_OF_ENCODERS]) -> Self {
        Self {
            encoders,
            encoders_state: [Keycode::KC_A; NUM_OF_ENCODERS],
        }
    }

    // initializes the encoder controller
    pub(super) fn initialize(&mut self) {
        self.encoders
            .iter_mut()
            .for_each(|encoder| encoder.initialize());
    }

    pub(super) fn periodic(&mut self) {
        self.encoders
            .iter_mut()
            .for_each(|encoder| encoder.update());
    }

    pub(super) fn actions(&self, layer: usize) -> [Keycode; NUM_OF_ENCODERS] {
        let mut actions = [Keycode::KC_NO; NUM_OF_ENCODERS];

        self.encoders
            .iter()
            .map(|encoder| encoder.action(layer))
            .enumerate()
            .for_each(|(index, action)| actions[index] = action);
        actions
    }
}
