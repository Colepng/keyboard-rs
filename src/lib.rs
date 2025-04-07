#![no_std]
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
pub mod debounce;

use cortex_m::prelude::_embedded_hal_watchdog_Watchdog;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
#[cfg(feature = "encoders")]
use hardware::encoder::Encoder;
use keyboard::Keyboard;
use keycode::Keycode;
use panic_halt as _;
use usb_device::class_prelude::UsbBus;
use usb_device::class_prelude::UsbBusAllocator;

#[cfg(feature = "rp2040")]
use fugit::ExtU32;
#[cfg(feature = "rp2040")]
use hal::{timer::CountDown as RPCountDown, Timer, Watchdog as RPWatchdog};
#[cfg(feature = "rp2040")]
use rp2040_hal as hal;
#[cfg(feature = "rp2040")]
use rp2040_hal::gpio::Pins;

// External high-speed crystal on the Raspberry Pi Pico board is 12MHz. Adjust
// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
// maybe remove the watchdog in the future

pub struct Board<Dog: _embedded_hal_watchdog_Watchdog, Usb: UsbBus> {
    usb_bus: Usb,
    watchdog: Dog,
}

#[cfg(feature = "rp2040")]
impl Board<rp2040_hal::Watchdog, rp2040_hal::usb::UsbBus> {
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
pub fn init() -> (Pins, Board<RPWatchdog, rp2040_hal::usb::UsbBus>, Timer) {
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

    let usb_bus = hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );

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
    Usb: UsbBus,
>(
    mut board: Board<Dog, Usb>,
    cols: &mut [Output],
    rows: &mut [Input],
    keys: &[&[&[Keycode]]],
    encoders: [Encoder<EncoderPin>; NUM_OF_ENCODERS],
    mut timer0: Timer,
    mut timer1: Timer,
    mut timer2: Timer,
) -> !
where
    [(); COLS * ROWS + { NUM_OF_ENCODERS }]: Sized, Timer::Time: From<fugit::Duration<u32, 1, 1000000>>
{
    // Set up the USB Communications Class Device driver

    let usb_bus = UsbBusAllocator::new(board.usb_bus);

    let mut keyboard =
        Keyboard::<COLS, ROWS, NUM_OF_ENCODERS, EncoderPin, Output, Input, Timer, Usb>::new(
            keys,
            cols,
            rows,
            encoders,
            &mut timer0,
            &mut timer1,
            &mut timer2,
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
    Usb: UsbBus,
>(
    mut board: Board<Dog, Usb>,
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
    let usb_bus = UsbBusAllocator::new(board.usb_bus);

    let mut keyboard = Keyboard::<COLS, ROWS, Output, Input, Timer, Usb>::new(
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
