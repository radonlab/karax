// Copyright (C) 2020, Skyler. All rights reserved.
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod arch;

use karax_macros::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // This function never returns
    loop {}
}
