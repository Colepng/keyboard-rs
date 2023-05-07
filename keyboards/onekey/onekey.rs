#![no_std]
#![no_main]

use keyboardrs::{init, matrix_scaning};
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::gpio::DynPin;

#[entry]
fn main() -> ! {
    const NUMOFCOL: usize = 1;
    const NUMOFROW: usize = 1;
    const KEYS: [[u8; NUMOFCOL]; NUMOFROW] = [[0x05]];

    let (pins, watchdog) = init();

    let col: [DynPin; NUMOFCOL] = [pins.gpio17.into()];
    let row: [DynPin; NUMOFROW] = [pins.gpio16.into()];

    matrix_scaning(col, row, KEYS, watchdog);
}
