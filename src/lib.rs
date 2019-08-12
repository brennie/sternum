// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proc_macro;

#[cfg(test)]
mod tests;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Sternum, attributes(sternum))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ts = quote! {};
    ts.into()
}
