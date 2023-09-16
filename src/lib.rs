#![no_std]
#![no_main]
#![feature(generic_const_exprs)]
#![feature(slice_flatten)]

#[cfg(feature = "encoders")]
pub mod hardware;
pub mod key;
mod keyboard;
pub mod keycode;
use cortex_m::delay::Delay;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::timer::CountDown;
use fugit::ExtU32;
use hal::clocks::ClocksManager;
use hal::pac::{interrupt, Peripherals, USBCTRL_DPRAM, USBCTRL_REGS};
use hal::usb::UsbBus;
use hal::{Clock, Timer, Watchdog};
#[cfg(feature = "encoders")]
use hardware::Encoder;
use keyboard::Keyboard;
use keycode::Keycodes;
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::gpio::{DynPin, Pins};
use rp2040_hal::timer;
use usb_device::UsbError;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};

use key::Key;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig;
use usbd_human_interface_device::device::DeviceHList;
use usbd_human_interface_device::usb_class::UsbHidClassBuilder;
use usbd_human_interface_device::{UsbHidError, page};

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
// maybe remove the watchdog in the future

pub struct Board {
    usb_bus: UsbBusAllocator<UsbBus>,
    timer: Timer,
    watchdog: Watchdog,
    delay: Delay,
}

pub fn init() -> (Pins, Board) {
    // setup peripherals
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // setup watchdog
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    watchdog.start(1.secs());

    // setup serial input/output
    let sio = hal::Sio::new(pac.SIO);
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

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

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let core = hal::pac::CorePeripherals::take().unwrap();
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    (
        pins,
        Board {
            usb_bus,
            timer,
            watchdog,
            delay,
        },
    )
}

pub fn matrix_scaning<
    const COLS: usize,
    const ROWS: usize,
    const LAYERS: usize,
    #[cfg(feature = "encoders")] const NUM_OF_ENCODERS: usize,
>(
    mut board: Board,
    mut cols: [DynPin; COLS],
    mut rows: [DynPin; ROWS],
    keys: [[[Keycodes; COLS]; ROWS]; LAYERS],
    #[cfg(feature = "encoders")] mut encoders: [Encoder<LAYERS>; NUM_OF_ENCODERS],
) -> ! {
    // Set up the USB Communications Class Device driver

    let mut usb_hid_class = UsbHidClassBuilder::new()
        .add_device(NKROBootKeyboardConfig::default())
        .build(&board.usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&board.usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Cole corp")
        .product("Keyboard")
        .serial_number("1")
        .build();

    rows.iter_mut().for_each(|pin| {
        pin.into_pull_down_input();
    });
    cols.iter_mut().for_each(|pin| {
        pin.into_readable_output();
    });

    let mut keyboard = Keyboard::<COLS, ROWS, LAYERS>::new();

    #[cfg(feature = "encoders")]
    for encoder in encoders.iter_mut() {
        encoder.channel_a.into_pull_up_input();
        encoder.channel_b.into_pull_up_input();
    }

    let mut input_count_down = board.timer.count_down();
    input_count_down.start(10.millis());

    let mut tick_count_down = board.timer.count_down();
    tick_count_down.start(1.millis());

    loop {
        // feed watchdog
        board.watchdog.feed();

        if input_count_down.wait().is_ok() {
            cols.iter_mut().enumerate().for_each(|(col, pin)| {
                pin.set_high().unwrap();

                rows.iter_mut().enumerate().for_each(|(row, pin)| {
                    if pin.is_high().unwrap() {
                        // on press
                        let key = Key {
                            col: Some(col),
                            row: Some(row),
                            keycode: keys[keyboard.layer][row][col],
                            encoder: false,
                        };
                        keyboard.key_press(key);
                    } else {
                        // on release
                        keyboard.key_release(keys, col, row);
                    }
                });

                pin.set_low().unwrap();
            });

            let state = keyboard.state();

            let keys = state.into_iter().map(|keycode| {
                page::Keyboard::from(
                    if let Some(keycode) = keycode {
                        keycode.try_into().unwrap_or(0)
                    } else {
                        0
                    }
                    )
            });
            
            match usb_hid_class.device().write_report(keys) {
                Err(UsbHidError::WouldBlock) => {}
                Err(UsbHidError::Duplicate) => {}
                Ok(_) => {}
                Err(e) => {
                    // core::panic!("Failed to write keyboard report: {:?}", e)
                }
            }

            #[cfg(feature = "encoders")]
            for encoder in encoders.iter_mut() {
                // keyboard.update_encoder(encoder, &mut delay);
            }
        }

        if tick_count_down.wait().is_ok() {
            match usb_hid_class.tick() {
                Err(UsbHidError::WouldBlock) => {}
                Ok(_) => {}
                Err(e) => {
                    // core::panic!("Failed to process keyboard tick: {:?}", e)
                }
            }
        }

        if usb_dev.poll(&mut [&mut usb_hid_class]) {
            match usb_hid_class.device().read_report() {
                Err(UsbError::WouldBlock) => {
                    //do nothing
                }
                Err(e) => {
                    core::panic!("Failed to read keyboard report: {:?}", e)
                }
                Ok(_) => {}
            }
        }

        // push_input_report(keyboard.report).ok().unwrap_or(0);
        // keyboard.update_state();
        // keyboard.reset();
    }
}

// fn push_input_report<T: AsInputReport>(report: T) -> Result<usize, usb_device::UsbError> {
//     critical_section::with(|_| unsafe {
//         // Now interrupts are disabled, grab the global variable and, if
//         // available, send it a HID report
//         USB_HID.as_mut().map(|hid| hid.push_input(&report))
//     })
//     .unwrap()
// }

// #[interrupt]
// unsafe fn USBCTRL_IRQ() {
//     let usb_hid = USB_HID.as_mut().unwrap();
//     let usb_device = USB_DEVICE.as_mut().unwrap();
//     usb_device.poll(&mut [usb_hid]);
// }
