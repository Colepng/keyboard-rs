#![no_std]
#![feature(slice_flatten)]
#![feature(generic_const_exprs)]
#![deny(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::nursery,
    clippy::cargo
)]

#[cfg(feature = "encoders")]
pub mod hardware;
mod keyboard;
pub mod keycode;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use hal::usb::UsbBus;
use hal::{Timer, Watchdog};
#[cfg(feature = "encoders")]
use hardware::encoder::Encoder;
use keyboard::Keyboard;
use keycode::Keycode;
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::gpio::Pins;
use usb_device::class_prelude::UsbBusAllocator;

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use fugit::ExtU32;

/// External high-speed crystal on the Raspberry Pi Pico board is 12MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
// maybe remove the watchdog in the future

pub struct Board {
    usb_bus: UsbBusAllocator<UsbBus>,
    timer: Timer,
    watchdog: Watchdog,
}

/// .
///
/// # Panics
///
/// Panics if something goes wrong in setup.
#[must_use]
pub fn init() -> (Pins, Board) {
    // setup peripherals
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // setup watchdog
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    watchdog.start(1.secs());

    // setup serial input/output
    let sio = hal::Sio::new(pac.SIO);

    // setup clock at 125MHz
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

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

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
    EncoderPin: InputPin,
    Output: OutputPin,
    Input: InputPin,
>(
    mut board: Board,
    cols: &mut [Output],
    rows: &mut [Input],
    keys: &[&[&[Keycode]]],
    encoders: [Encoder<EncoderPin>; NUM_OF_ENCODERS],
) -> !
where
    [(); COLS * ROWS + { NUM_OF_ENCODERS }]: Sized,
{
    // Set up the USB Communications Class Device driver

    let timer = board.timer;
    let usb_bus = board.usb_bus;

    let mut keyboard = Keyboard::<COLS, ROWS, NUM_OF_ENCODERS, EncoderPin, Output, Input>::new(
        keys, cols, rows, encoders, &timer, &usb_bus,
    );

    keyboard.initialize();

    loop {
        // feed watchdog
        board.watchdog.feed();

        keyboard.periodic();
    }
}

#[cfg(not(feature = "encoders"))]
pub fn matrix_scaning<
    const COLS: usize,
    const ROWS: usize,
    const LAYERS: usize,
    Output: OutputPin,
    Input: InputPin,
>(
    mut board: Board,
    cols: &mut [Output],
    rows: &mut [Input],
    keys: &[&[&[Keycode]]],
) -> !
where
    [(); COLS * ROWS]: Sized,
{
    // Set up the USB Communications Class Device driver

    let timer = board.timer;
    let usb_bus = board.usb_bus;

    let mut keyboard =
        Keyboard::<COLS, ROWS, Output, Input>::new(keys, cols, rows, &timer, &usb_bus);

    keyboard.initialize();

    loop {
        // feed watchdog
        board.watchdog.feed();

        keyboard.periodic();
    }
}
