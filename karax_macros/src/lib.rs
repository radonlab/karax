// Copyright (C) 2020, Skyler. All rights reserved.
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let main_fn = parse_macro_input!(input as ItemFn);
    let ident = &main_fn.sig.ident;
    quote! {
        #main_fn

        pub unsafe extern "C" fn kmain() -> ! {
            #ident()
        }
    }
    .into()
}
