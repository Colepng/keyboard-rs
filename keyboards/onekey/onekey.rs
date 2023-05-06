#![no_std]
#![no_main]

use rp_pico::entry;
use panic_halt as _;
use keyboardrs::init;
#[entry]
fn main() -> ! {
    init();
}
