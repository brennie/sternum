// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proc_macro;

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

    if variants.len() == 0 {
        return Err(Error::new_spanned(
            ast,
            "Sternum only supports enums with at least one variant",
        )
        .into());
    }

    {
        let variant_errors: Vec<Error> = variants
            .iter()
            .filter_map(|variant| match variant.fields {
                syn::Fields::Unit => None,
                syn::Fields::Named(..) | syn::Fields::Unnamed(..) => Some(Error::new_spanned(
                    variant,
                    "Sternum only supports unit enum variants (like Option::None)",
                )),
            })
            .collect();

        if variant_errors.len() != 0 {
            return Err(ErrorList(variant_errors));
        }
    }

    Ok(impl_display(&ast.ident, variants.iter()))
}

fn impl_display<'a, I>(name: &syn::Ident, variants: I) -> TokenStream
where
    I: Iterator<Item = &'a syn::Variant>,
{
    let matches = variants.map(|variant| {
        let ident = &variant.ident;
        let repr: syn::Lit = syn::LitStr::new(&ident.to_string(), ident.span()).into();

        quote! {
            #name::#ident => write!(f, "{}", #repr),
        }
    });

    let quoted = quote! {
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    #(#matches)*

                }
            }
        }
    };

    quoted.into()
}
