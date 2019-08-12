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
use syn::{parse_macro_input, DeriveInput, Error};

struct ErrorList(Vec<Error>);

impl ErrorList {
    fn to_compile_error(self) -> TokenStream {
        let errors = self.0.iter().map(Error::to_compile_error);

        let quoted = quote! {
            #(#errors)*
        };

        quoted.into()
    }
}

impl From<Error> for ErrorList {
    fn from(e: Error) -> Self {
        ErrorList(vec![e])
    }
}

#[proc_macro_derive(Sternum, attributes(sternum))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match derive_impl(&ast) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_impl(ast: &DeriveInput) -> Result<TokenStream, ErrorList> {
    let variants = match &ast.data {
        syn::Data::Enum(syn::DataEnum { ref variants, .. }) => variants,
        _ => {
            return Err(Error::new_spanned(ast, "Sternum only supports enums").into());
        }
    };

    let ts = quote! {};

    Ok(ts.into())
}
