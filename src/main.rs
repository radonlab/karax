#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use panic_halt as _;
use cortex_a_rt::entry;

#[entry]
fn main() -> ! {
    // Diverging main function never returns
    loop {}
}
