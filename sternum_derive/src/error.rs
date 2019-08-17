// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

#[derive(Debug)]
pub struct ErrorList(pub Vec<Error>);

impl ErrorList {
    /// Transform the list of errors into a list of compile errors.
    pub fn to_compile_error(self) -> TokenStream {
        let errors = self.0.iter().map(Error::to_compile_error);

        quote! {
            #(#errors)*
        }
    }
}

impl From<Error> for ErrorList {
    fn from(e: Error) -> Self {
        ErrorList(vec![e])
    }
}
