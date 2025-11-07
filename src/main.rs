#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use panic_halt as _;

#[entry]
fn main() -> ! {
    // Diverging main function never returns
    loop {}
}
