// Copyright (C) 2020, Skyler. All rights reserved.
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

.section ".text.boot"
.global _start

// fn _start()
_start:
    // Jump to Rust main function
    bl kmain
