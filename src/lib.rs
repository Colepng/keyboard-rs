#![no_std]
#![no_main]

pub mod config;
#[cfg(feature="encoders")]
pub mod hardware;
pub mod keycode;
// use config::Config;
use cortex_m::delay::Delay;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use fugit::ExtU32;
use hal::pac::interrupt;
use hal::Clock;
#[cfg(feature="encoders")]
use hardware::{Dir, Encoder};
use keycode::{Keycodes, Modifers};
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::gpio::{DynPin, Pins};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};

// USB Human Interface Device (HID) Class support
// use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::descriptor::SerializedDescriptor;
use usbd_hid::hid_class;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<hid_class::HIDClass<hal::usb::UsbBus>> = None;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// maybe remove the watchdog in the future
pub fn init() -> (Pins, hal::Watchdog, Delay) {
    // setup peripherals
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // setup watchdog
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    watchdog.start(1.secs());

    // setup serial input/output
    let sio = hal::Sio::new(pac.SIO);

    // setup clock at 125Mhz
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the USB Communications Class Device driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Setup usb hid class
    let usb_hid = hid_class::HIDClass::new_ep_in_with_settings(
        bus_ref,
        KeyboardReport::desc(),
        60,
        hid_class::HidClassSettings {
            subclass: hid_class::HidSubClass::NoSubClass,
            config: hid_class::ProtocolModeConfig::ForceReport,
            locale: hid_class::HidCountryCode::US,
            protocol: hid_class::HidProtocol::Keyboard,
        },
    );

    unsafe {
        USB_HID = Some(usb_hid);
    }

    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
        .manufacturer("Cole corp")
        .product("One Key")
        .serial_number("1")
        .device_class(0)
        .build();

    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        hal::pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    let core = hal::pac::CorePeripherals::take().unwrap();
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    (pins, watchdog, delay)
}

pub fn matrix_scaning<const COLS: usize, const ROWS: usize, const LAYERS: usize, 
    #[cfg(feature="encoders")]
    const NUM_OF_ENCODERS: usize>(
    mut cols: [DynPin; COLS],
    mut rows: [DynPin; ROWS],
    keys: [[[Keycodes; COLS]; ROWS]; LAYERS],
    #[cfg(feature="encoders")]
    mut encoders: [Encoder<LAYERS>; NUM_OF_ENCODERS],
    // #[cfg(feature="encoders")]
    // config: Config,
    mut watchdog: hal::Watchdog,
    mut delay: Delay,
) -> ! {
    rows.iter_mut().for_each(|pin| {
        pin.into_pull_down_input();
    });
    cols.iter_mut().for_each(|pin| {
        pin.into_readable_output();
    });

    let mut current_state = [[false; COLS]; ROWS];
    let mut old_current_state = current_state;
    let mut locked_keys = [[(false, 0); COLS]; ROWS];

    #[cfg(feature="encoders")]
    for encoder in encoders.iter_mut() {
        encoder.channel_a.into_pull_up_input();
        encoder.channel_b.into_pull_up_input();
    }

    let mut layer: usize = 0;
    let mut last_layer: usize = 0;
    let mut index: usize;
    let mut report: KeyboardReport;
    loop {
        // feed watchdog
        watchdog.feed();
        index = 0;

        report = KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x00; 6],
        };
        
        for (col, pin) in cols.iter_mut().enumerate() {
            pin.set_high().unwrap();
            for (row, pin) in rows.iter_mut().enumerate() {
                if index <= 6 && pin.is_high().unwrap() {
                    current_state[row][col] = true;
                    // on press
                    if !locked_keys[row][col].0 {
                        match keys[layer][row][col] {
                            Keycodes::KC_MO(x) => {
                                // if !e[row][col] {
                                locked_keys[row][col] = (true, layer);
                                last_layer = layer;
                                layer = x;
                            }
                            Keycodes::KC_LEFT_CTRL => {
                                    report.modifier |= Modifers::MOD_LCTRL as u8;
                            }
                            Keycodes::KC_LEFT_SHIFT => {
                                    report.modifier |= Modifers::MOD_LSHIFT as u8;
                            }
                            Keycodes::KC_LEFT_ALT => {
                                    report.modifier |= Modifers::MOD_LALT as u8;
                            }
                            Keycodes::KC_LEFT_GUI => {
                                    report.modifier |= Modifers::MOD_LGUI as u8;
                            }
                            Keycodes::KC_RIGHT_CTRL => {
                                    report.modifier |= Modifers::MOD_LCTRL as u8;
                            }
                            Keycodes::KC_RIGHT_SHIFT => {
                                    report.modifier |= Modifers::MOD_RSHIFT as u8;
                            }
                            Keycodes::KC_RIGHT_ALT => {
                                    report.modifier |= Modifers::MOD_RALT as u8;
                            }
                            Keycodes::KC_RIGHT_GUI => {
                                    report.modifier |= Modifers::MOD_RGUI as u8;
                            }
                            _ => {
                                if let Ok(keycode) = keys[layer][row][col].try_into() {
                                    report.keycodes[index] = keycode;
                                    index += 1;
                                }
                            }
                        }
                    }
                } else {
                    // on release
                    if old_current_state[row][col] {
                        match keys[locked_keys[row][col].1][row][col] {
                            Keycodes::KC_LAYER(x) => layer = x,
                            Keycodes::KC_MO(_) => { 
                                layer = last_layer;
                                locked_keys[row][col] = (false, layer);
                            }
                            _ => {}
                        }
                    }
                    current_state[row][col] = false;
                }
            }
            pin.set_low().unwrap();
        }


        #[cfg(feature="encoders")]
        for encoder in encoders.iter_mut() {
            encoder.update();
            match encoder.dir {
                Dir::Cw => {
                    // report.keycodes[index] = 0x1e;
                    let keycode = encoder.actions_clock_wise[layer];
                    if let Ok(keycode) = keycode.try_into() {
                        report.keycodes[index] = keycode;
                    } else {
                        match keycode {
                            Keycodes::KC_LAYER(x) => layer = x as usize,
                            _ => {}
                        }
                    }
                    delay.delay_ms(50);
                    push_keyboard_inputs(report).ok().unwrap_or(0);
                    report.keycodes[index] = 0x00;
                }
                Dir::Cww => {
                    // report.keycodes[index] = 0x1f;
                    let keycode = encoder.actions_counter_clock_wise[layer];
                    if let Ok(keycode) = keycode.try_into() {
                        report.keycodes[index] = keycode;
                    } else {
                        match keycode {
                            Keycodes::KC_LAYER(x) => layer = x as usize,
                            _ => {}
                        }
                    }
                    delay.delay_ms(50);
                    push_keyboard_inputs(report).ok().unwrap_or(0);
                    report.keycodes[index] = 0x00;
                }
                _ => {}
            }
        }

        push_keyboard_inputs(report).ok().unwrap_or(0);
        old_current_state = current_state;
    }
}

fn push_keyboard_inputs(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    critical_section::with(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}

#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_hid = USB_HID.as_mut().unwrap();
    let usb_device = USB_DEVICE.as_mut().unwrap();
    usb_device.poll(&mut [usb_hid]);
}
