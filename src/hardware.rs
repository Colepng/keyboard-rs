use rp_pico::hal::gpio::DynPin;

use crate::keycode::Keycodes;

pub struct Encoder {
    pub channel_a: DynPin,
    pub channel_b: DynPin,
    pub action_clock_wise: Keycodes,
    pub action_counter_clock_wise: Keycodes,
}
