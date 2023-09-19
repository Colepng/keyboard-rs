#![no_std]
#![no_main]
#![feature(slice_flatten)]
#![feature(generic_const_exprs)]

#[cfg(feature = "encoders")]
pub mod hardware;
pub mod key;
mod keyboard;
pub mod keycode;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use hal::usb::UsbBus;
use hal::{Timer, Watchdog};
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

    (
        pins,
        Board {
            usb_bus,
            timer,
            watchdog,
        },
    )
}

#[cfg(feature = "encoders")]
pub fn matrix_scaning<
    const COLS: usize,
    const ROWS: usize,
    const LAYERS: usize,
    const NUM_OF_ENCODERS: usize,
>(
    mut board: Board,
    cols: &mut [DynPin],
    rows: &mut [DynPin],
    keys: &[&[&[Keycode]]],
    encoders: [Encoder; NUM_OF_ENCODERS],
) -> !
where
    [(); COLS * ROWS + { NUM_OF_ENCODERS }]: Sized,
{
    // Set up the USB Communications Class Device driver

    let timer = board.timer;
    let usb_bus = board.usb_bus;

    let mut keyboard =
        Keyboard::<COLS, ROWS, NUM_OF_ENCODERS>::new(keys, cols, rows, encoders, &timer, &usb_bus);

    keyboard.initialize();

    loop {
        // feed watchdog
        board.watchdog.feed();

        keyboard.periodic();
    }
}

#[cfg(not(feature = "encoders"))]
pub fn matrix_scaning<const COLS: usize, const ROWS: usize, const LAYERS: usize>(
    mut board: Board,
    cols: &mut [DynPin],
    rows: &mut [DynPin],
    keys: &[&[&[Keycode]]],
) -> !
where
    [(); COLS * ROWS]: Sized,
{
    // Set up the USB Communications Class Device driver

    let timer = board.timer;
    let usb_bus = board.usb_bus;

    let mut keyboard = Keyboard::<COLS, ROWS>::new(keys, cols, rows, &timer, &usb_bus);

    keyboard.initialize();

    loop {
        // feed watchdog
        board.watchdog.feed();

        keyboard.periodic();
    }
}
