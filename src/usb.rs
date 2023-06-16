use usbd_hid::descriptor::{gen_hid_descriptor, AsInputReport};
use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
    },
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        (usage_page = CONSUMER, usage_min = 0x00, usage_max = 0x514) = {
            #[item_settings data,array,absolute,not_null] consumer_control=input;
        }
    }
)]

pub struct Report {
    pub modifier: u8,
    pub keycodes: [u8; 10],
    pub consumer_control: u8,
}

impl Default for Report {
    fn default() -> Self {
        Report { modifier: 0x00, keycodes: [0x00; 10], consumer_control: 0x00 }
    }
}
