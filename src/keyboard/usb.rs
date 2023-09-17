use embedded_hal::timer::CountDown;
use frunk::HList;
use fugit::ExtU32;
use rp2040_hal::{timer::CountDown as RPCountDown, usb::UsbBus, Timer};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_human_interface_device::{
    device::keyboard::{NKROBootKeyboard, NKROBootKeyboardConfig},
    page,
    usb_class::{UsbHidClass, UsbHidClassBuilder},
    UsbHidError,
};

use crate::keycode::Keycode;

pub(super) struct Usb<'a> {
    usb_dev: UsbDevice<'a, UsbBus>,
    usb_hid_class: UsbHidClass<'a, UsbBus, HList!(NKROBootKeyboard<'a, UsbBus>)>,
    usb_tick_timer: RPCountDown<'a>,
}

impl<'a> Usb<'a> {
    pub(super) fn new(usb_bus: &'a UsbBusAllocator<UsbBus>, timer: &'a Timer) -> Self {
        let usb_hid_class = UsbHidClassBuilder::new()
            .add_device(NKROBootKeyboardConfig::default())
            .build(usb_bus);

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
            .manufacturer("Cole corp")
            .product("Keyboard")
            .serial_number("1")
            .build();

        let usb_tick_timer = timer.count_down();

        Self {
            usb_dev,
            usb_hid_class,
            usb_tick_timer,
        }
    }

    pub(super) fn initialize(&mut self) {
        self.usb_tick_timer.start(10.millis());
    }

    pub(super) fn periodic(&mut self) {
        // tick usb class
        if self.usb_tick_timer.wait().is_ok() {
            match self.usb_hid_class.tick() {
                Err(UsbHidError::WouldBlock) => {}
                Ok(_) => {}
                Err(e) => {
                    core::panic!("Failed to process keyboard tick: {:?}", e)
                }
            }
        }

        // poll usb device
        if self.usb_dev.poll(&mut [&mut self.usb_hid_class]) {
            match self.usb_hid_class.device().read_report() {
                Err(UsbError::WouldBlock) => {
                    //do nothing
                }
                Err(e) => {
                    core::panic!("Failed to read keyboard report: {:?}", e)
                }
                Ok(_) => {}
            }
        }
    }

    pub(super) fn write_report(&mut self, keys: &[Keycode]) {
        let keys = keys
            .into_iter()
            .map(|keycode| page::Keyboard::from(keycode.try_into().unwrap_or(0)));

        match self.usb_hid_class.device().write_report(keys) {
            Err(UsbHidError::WouldBlock) => {}
            Err(UsbHidError::Duplicate) => {}
            Ok(_) => {}
            Err(e) => {
                core::panic!("Failed to write keyboard report: {:?}", e)
            }
        }
    }

    // pub(super) fn should_write_report(&mut self) -> bool {
    //     self.usb_tick_timer.wait().is_ok()
    // }
}
