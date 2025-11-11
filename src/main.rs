#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod arch;

use panic_halt as _;
use arch::arm64::boot::entry;

#[entry]
fn main() -> ! {
    // Diverging main function never returns
    loop {}
}
