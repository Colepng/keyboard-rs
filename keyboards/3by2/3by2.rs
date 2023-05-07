#![no_std]
#![no_main]

use keyboardrs::{init, matrix_scaning};
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 3;
    const NUMOFROW: usize = 2;
    const KEYS: [[u8; NUMOFCOL]; NUMOFROW] = [[0x04, 0x05, 0x06], [0x12, 0x34, 0x1f]];

    let (pins, watchdog) = init();

    let col: [DynPin; NUMOFCOL] = [pins.gpio28.into(), pins.gpio26.into(), pins.gpio17.into()];
    let row: [DynPin; NUMOFROW] = [pins.gpio16.into(), pins.gpio15.into()];

    matrix_scaning(col, row, KEYS, watchdog);
}
