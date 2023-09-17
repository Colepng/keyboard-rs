#![no_std]
#![no_main]
#![feature(slice_flatten)]

#[cfg(feature = "encoders")]
pub mod hardware;
pub mod key;
mod keyboard;
pub mod keycode;
use cortex_m::delay::Delay;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use hal::usb::UsbBus;
use hal::{Clock, Timer, Watchdog};
#[cfg(feature = "encoders")]
use hardware::Encoder;
use keyboard::Keyboard;
use keycode::Keycode;
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::gpio::{DynPin, Pins};
use usb_device::class_prelude::UsbBusAllocator;

use fugit::ExtU32;

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
    cols: &mut [DynPin],
    rows: &mut [DynPin],
    keys: &[&[&[Keycode]]],
    #[cfg(feature = "encoders")] mut encoders: [Encoder<LAYERS>; NUM_OF_ENCODERS],
) -> ! {
    // Set up the USB Communications Class Device driver

    let mut keyboard = Keyboard::<COLS, ROWS>::new(keys, cols, rows, &board.timer, &board.usb_bus);

    keyboard.initialize();

    #[cfg(feature = "encoders")]
    for encoder in encoders.iter_mut() {
        encoder.channel_a.into_pull_up_input();
        encoder.channel_b.into_pull_up_input();
    }

    loop {
        // feed watchdog
        board.watchdog.feed();

        keyboard.periodic();

        #[cfg(feature = "encoders")]
        for encoder in encoders.iter_mut() {
            // keyboard.update_encoder(encoder, &mut delay);
        }
    }
}
