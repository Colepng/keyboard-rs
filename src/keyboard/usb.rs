use embedded_hal::timer::CountDown;
use frunk::HList;
use fugit::ExtU32;
use rp2040_hal::usb::UsbBus;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_human_interface_device::{
    device::{
        consumer::{
            ConsumerControl, ConsumerControlConfig, MultipleConsumerReport,
            MULTIPLE_CODE_REPORT_DESCRIPTOR,
        },
        keyboard::{NKROBootKeyboard, NKROBootKeyboardConfig},
    },
    interface::InterfaceBuilder,
    page,
    usb_class::{UsbHidClass, UsbHidClassBuilder},
    UsbHidError,
};

type HidClass<'a> =
    UsbHidClass<'a, UsbBus, HList!(ConsumerControl<'a, UsbBus>, NKROBootKeyboard<'a, UsbBus>)>;

use crate::keycode::Keycode;

pub(super) struct Usb<'a, Timer: CountDown> {
    usb_dev: UsbDevice<'a, UsbBus>,
    usb_hid_class: HidClass<'a>,
    usb_tick_timer: &'a mut Timer,
    last_consumer_report: MultipleConsumerReport,
}

impl<'a, Timer: CountDown> Usb<'a, Timer> {
    pub(super) fn new(usb_bus: &'a UsbBusAllocator<UsbBus>, timer: &'a mut Timer) -> Self {
        let usb_hid_class = UsbHidClassBuilder::new()
            .add_device(NKROBootKeyboardConfig::default())
            .add_device(ConsumerControlConfig::new(
                InterfaceBuilder::new(MULTIPLE_CODE_REPORT_DESCRIPTOR)
                    .unwrap()
                    .description("Consumer Control")
                    .in_endpoint(10.millis())
                    .unwrap()
                    .without_out_endpoint()
                    .build(),
            ))
            .build(usb_bus);

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
            .manufacturer("Cole corp")
            .product("Keyboard, Keyboard Conusmer")
            .serial_number("1")
            .build();

        let usb_tick_timer = timer;

        Self {
            usb_dev,
            usb_hid_class,
            usb_tick_timer,
            last_consumer_report: MultipleConsumerReport::default(),
        }
    }

    // pub(super) fn initialize(&mut self) {
    // }

    pub(super) fn periodic(&mut self) {
        // tick usb class
        if self.usb_tick_timer.wait().is_ok() {
            match self.usb_hid_class.tick() {
                Err(UsbHidError::WouldBlock) | Ok(()) => {}
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
                Err(UsbError::WouldBlock) | Ok(_) => {}
                Err(e) => {
                    core::panic!("Failed to read keyboard report: {:?}", e)
                }
            }
        }
    }

    pub(super) fn write_keyboard_report(&mut self, keys: &[Keycode]) {
        let keyboard = keys
            .iter()
            .filter(|keycode| !keycode.is_consumer())
            .flat_map(|keycode| {
                if let Keycode::KEYS_2(key1, key2) = keycode {
                    [
                        page::Keyboard::from((**key1).try_into().unwrap_or(0)),
                        page::Keyboard::from((**key2).try_into().unwrap_or(0)),
                    ]
                } else {
                    [
                        page::Keyboard::from(keycode.try_into().unwrap_or(0)),
                        page::Keyboard::ErrorUndefine,
                    ]
                }
            });

        match self
            .usb_hid_class
            .device::<NKROBootKeyboard<'_, _>, _>()
            .write_report(keyboard)
        {
            Err(UsbHidError::WouldBlock) | Err(UsbHidError::Duplicate) | Ok(_) => {}
            Err(e) => {
                core::panic!("Failed to write keyboard report: {:?}", e)
            }
        }
    }

    pub(super) fn write_consumer_report(&mut self, keys: &[Keycode]) {
        let mut consumer_array = [page::Consumer::Unassigned; 4];
        keys.iter()
            .filter_map(|keycode| {
                if keycode.is_consumer() {
                    Some(if let Keycode::KEYS_2(key1, key2) = keycode {
                        [
                            page::Consumer::from((**key1).into_consumer().unwrap_or(0)),
                            page::Consumer::from((**key2).into_consumer().unwrap_or(0)),
                        ]
                    } else {
                        [
                            page::Consumer::from(keycode.into_consumer().unwrap_or(0)),
                            page::Consumer::Unassigned,
                        ]
                    })
                } else {
                    None
                }
            })
            .flatten()
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
}
