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
#![allow(clippy::multiple_crate_versions)]

#[cfg(feature = "encoders")]
pub mod hardware;
mod keyboard;
pub mod keycode;
use cortex_m::prelude::{_embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable};
use hal::usb::UsbBus;

#[cfg(feature = "rp2040")]
use hal::{timer::CountDown as RPCountDown, Timer, Watchdog as RPWatchdog};
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
use embedded_hal::timer::CountDown;
use fugit::ExtU32;

#[cfg(feature = "rp2040")]
// External high-speed crystal on the Raspberry Pi Pico board is 12MHz. Adjust
// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
// maybe remove the watchdog in the future

pub struct Board<Dog: _embedded_hal_watchdog_Watchdog> {
    usb_bus: UsbBusAllocator<UsbBus>,
    watchdog: Dog,
}

#[cfg(feature = "rp2040")]
impl Board<rp2040_hal::Watchdog> {
    pub fn setup_timers<'a>(
        timer0: &'a Timer,
        timer1: &'a Timer,
    ) -> (RPCountDown<'a>, RPCountDown<'a>) {
        let mut timer0 = timer0.count_down();
        timer0.start(10.millis());
        let mut timer1 = timer1.count_down();
        timer1.start(1.millis());
        (timer0, timer1)
    }
}

#[cfg(feature = "rp2040")]
/// .
///
/// # Panics
///
/// Panics if something goes wrong in setup.
#[must_use]
pub fn init() -> (Pins, Board<RPWatchdog>, Timer) {
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

    let board = Board { usb_bus, watchdog };

    (pins, board, timer)
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
    Timer: CountDown,
    Dog: _embedded_hal_watchdog_Watchdog,
>(
    mut board: Board<Dog>,
    cols: &mut [Output],
    rows: &mut [Input],
    keys: &[&[&[Keycode]]],
    encoders: [Encoder<EncoderPin>; NUM_OF_ENCODERS],
    mut timer0: Timer,
    mut timer1: Timer,
) -> !
where
    [(); COLS * ROWS + { NUM_OF_ENCODERS }]: Sized,
{
    // Set up the USB Communications Class Device driver

    let usb_bus = board.usb_bus;

    let mut keyboard =
        Keyboard::<COLS, ROWS, NUM_OF_ENCODERS, EncoderPin, Output, Input, Timer>::new(
            keys,
            cols,
            rows,
            encoders,
            &mut timer0,
            &mut timer1,
            &usb_bus,
        );

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
    Timer: CountDown,
    Dog: _embedded_hal_watchdog_Watchdog,
>(
    mut board: Board<Dog>,
    cols: &mut [Output],
    rows: &mut [Input],
    keys: &[&[&[Keycode]]],
    mut timer0: Timer,
    mut timer1: Timer,
) -> !
where
    [(); COLS * ROWS]: Sized,
{
    // Set up the USB Communications Class Device driver

    // let timer = board.timer;
    let usb_bus = board.usb_bus;

    let mut keyboard = Keyboard::<COLS, ROWS, Output, Input, Timer>::new(
        keys,
        cols,
        rows,
        &mut timer0,
        &mut timer1,
        &usb_bus,
    );

    loop {
        // feed watchdog
        board.watchdog.feed();

        keyboard.periodic();
    }
}
