// Copyright (C) 2020, Skyler. All rights reserved.
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ItemFn, parse_macro_input};

fn validate_fn_signature(fn_sig: &syn::Signature) -> Result<(), Error> {
    if fn_sig.inputs.len() != 0 {
        return Err(Error::new_spanned(
            fn_sig,
            "The entry function should take no arguments",
        ));
    }

    if matches!(fn_sig.output, syn::ReturnType::Default) {
        return Err(Error::new_spanned(
            fn_sig,
            "The entry function should return ! (never type)",
        ));
    }

    Ok(())
}

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let main_fn = parse_macro_input!(input as ItemFn);
    let ident = &main_fn.sig.ident;
    // check the function signature
    if let Err(error) = validate_fn_signature(&main_fn.sig) {
        return error.to_compile_error().into();
    }
    quote! {
        #main_fn

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn kmain() -> ! {
            #ident()
        }
    }
    .into()
}
