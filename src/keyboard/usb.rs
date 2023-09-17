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
    device::{
        consumer::{ConsumerControl, ConsumerControlConfig, MultipleConsumerReport},
        keyboard::{NKROBootKeyboard, NKROBootKeyboardConfig},
    },
    page,
    usb_class::{UsbHidClass, UsbHidClassBuilder},
    UsbHidError,
};

use crate::keycode::Keycode;

pub(super) struct Usb<'a> {
    usb_dev: UsbDevice<'a, UsbBus>,
    usb_hid_class:
        UsbHidClass<'a, UsbBus, HList!(ConsumerControl<'a, UsbBus>, NKROBootKeyboard<'a, UsbBus>)>,
    usb_tick_timer: RPCountDown<'a>,
    usb_consumer_timer: RPCountDown<'a>,
    last_consumer_report: MultipleConsumerReport,
}

impl<'a> Usb<'a> {
    pub(super) fn new(usb_bus: &'a UsbBusAllocator<UsbBus>, timer: &'a Timer) -> Self {
        let usb_hid_class = UsbHidClassBuilder::new()
            .add_device(NKROBootKeyboardConfig::default())
            .add_device(ConsumerControlConfig::default())
            .build(usb_bus);

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
            .manufacturer("Cole corp")
            .product("Keyboard, Keyboard Conusmer")
            .serial_number("1")
            .build();

        let usb_tick_timer = timer.count_down();

        let usb_consumer_timer = timer.count_down();

        Self {
            usb_dev,
            usb_hid_class,
            usb_tick_timer,
            usb_consumer_timer,
            last_consumer_report: MultipleConsumerReport::default(),
        }
    }

    pub(super) fn initialize(&mut self) {
        self.usb_tick_timer.start(10.millis());
        self.usb_consumer_timer.start(50.millis());
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
            match self
                .usb_hid_class
                .device::<NKROBootKeyboard<'_, _>, _>()
                .read_report()
            {
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

    pub(super) fn write_keyboard_report(&mut self, keys: &[Keycode]) {
        let keyboard = keys
            .into_iter()
            .filter(|keycode| !keycode.is_consumer())
            .map(|keycode| page::Keyboard::from(keycode.try_into().unwrap_or(0)));

        match self
            .usb_hid_class
            .device::<NKROBootKeyboard<'_, _>, _>()
            .write_report(keyboard)
        {
            Err(UsbHidError::WouldBlock) => {}
            Err(UsbHidError::Duplicate) => {}
            Ok(_) => {}
            Err(e) => {
                core::panic!("Failed to write keyboard report: {:?}", e)
            }
        }
    }

    pub(super) fn write_consumer_report(&mut self, keys: &[Keycode]) {
        let mut consumer_array = [page::Consumer::Unassigned; 4];
        keys.into_iter()
            .filter_map(|keycode| {
                if keycode.is_consumer() {
                    Some(page::Consumer::from(keycode.into_consumer().unwrap_or(0)))
                } else {
                    None
                }
            })
            .enumerate()
            .for_each(|(index, consumer)| {
                if index < 4 {
                    consumer_array[index] = consumer;
                }
            });

        let consumer_report = MultipleConsumerReport {
            codes: consumer_array,
        };

        if self.last_consumer_report != consumer_report {
            match self
                .usb_hid_class
                .device::<ConsumerControl<'_, _>, _>()
                .write_report(&consumer_report)
            {
                Err(UsbError::WouldBlock) => {}
                Ok(_) => {
                    self.last_consumer_report = consumer_report;
                }
                Err(e) => {
                    core::panic!("Failed to write keyboard report: {:?}", e)
                }
            }
        }
    }

    pub(super) fn should_write_consumer_report(&mut self) -> bool {
        self.usb_consumer_timer.wait().is_ok()
    }

    // pub(super) fn should_write_report(&mut self) -> bool {
    //     self.usb_tick_timer.wait().is_ok()
    // }
}
