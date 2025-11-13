// Copyright (C) 2020, Skyler. All rights reserved.
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let modified = format!(
        "#[export_name = \"_start_rust\"]
         {}
         ",
        input
    );
    modified.parse().unwrap()
}
