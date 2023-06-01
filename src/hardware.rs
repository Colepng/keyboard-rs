use rp2040_hal::gpio::DynPin;

use crate::keycode::Keycodes;

pub struct Encoder<const LAYERS: usize> {
    pub channel_a: DynPin,
    pub channel_b: DynPin,
    pub actions_clock_wise: [Keycodes; LAYERS],
    pub actions_counter_clock_wise: [Keycodes; LAYERS],
}
