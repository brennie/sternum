// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "128"]

mod error;
mod features;

extern crate proc_macro;

use std::collections::HashMap;
use std::convert::identity;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Ident};

use crate::error::ErrorList;
use crate::features::{parse_features, FeatureSet, TransformKind};

/// The custom derive for the [`Sternum`][sternum::Sternum] trait.
///
/// Deriving this trait will also derive [`Display`][std::fmt::Display] and
/// [`FromStr`][std::str::FromStr] implementations.
///
/// [sternum::Sternum]: ../sternum/trait.Sternum.html
/// [std::fmt::Display]: https://doc.rust-lang.org/std/std/trait.Display.html
/// [std::str::FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
#[proc_macro_derive(Sternum, attributes(sternum))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match derive_impl(&ast) {
        Ok(ts) => ts.into(),
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

    let features = parse_features(&ast.attrs)?;

    if features.case_insensitive || features.transform.is_some() {
        let variant_errors: Vec<Error> = variants
            .iter()
            .scan(HashMap::<String, &Ident>::new(), |variant_names, variant| {
                let name = variant.ident.to_string().to_lowercase();

                if let Some(ref prev_ident) = variant_names.get(&name) {
                    Some(Some(Error::new_spanned(
                        &variant.ident,
                        format!("The variant `{}' is a case-insensitive match of a previous identifier (`{}')",
                            variant.ident.to_string(),
                            prev_ident.to_string(),
                        ))))
                } else {
                    variant_names.insert(name, &variant.ident);
                    Some(None)
                }
            })
            .filter_map(identity)
            .collect();

        if variant_errors.len() != 0 {
            return Err(ErrorList(variant_errors));
        }
    }

    let sternum_impl = impl_sternum(&ast.ident);
    let display_impl = impl_display(&ast.ident, variants.iter(), &features);
    let from_str_impl = impl_from_str(&ast.ident, variants.iter(), &features);

    let quoted = quote! {
        #sternum_impl
        #display_impl
        #from_str_impl
    };

    Ok(quoted.into())
}

fn impl_sternum(type_name: &syn::Ident) -> TokenStream {
    let type_name_as_str = type_name.to_string();

    quote! {
        impl ::sternum::Sternum for #type_name {
            fn type_name() -> &'static str {
                return #type_name_as_str;
            }
        }

    }
}

fn impl_display<'a, I>(type_name: &syn::Ident, variants: I, features: &FeatureSet) -> TokenStream
where
    I: Iterator<Item = &'a syn::Variant>,
{
    let matches = variants.map(|variant| {
        let ident = &variant.ident;

        let repr = if features.scoped {
            format!("{}::{}", type_name, ident)
        } else {
            ident.to_string()
        };

        let repr = if let Some(ref trans) = &features.transform {
            match trans {
                TransformKind::Uppercase => repr.to_uppercase(),
                TransformKind::Lowercase => repr.to_lowercase(),
            }
        } else {
            repr
        };

        let repr: syn::Lit = syn::LitStr::new(&repr, ident.span()).into();

        quote! {
            #type_name::#ident => write!(f, #repr),
        }
    });

    quote! {
        impl ::std::fmt::Display for #type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    #(#matches)*
                }
            }
        }
    }
}

fn impl_from_str<'a, I>(type_name: &syn::Ident, variants: I, features: &FeatureSet) -> TokenStream
where
    I: Iterator<Item = &'a syn::Variant>,
{
    let matches = variants.map(|variant| {
        let ident = &variant.ident;
        let repr = if features.scoped {
            format!("{}::{}", type_name, ident)
        } else {
            ident.to_string()
        };

        let repr = match (&features.case_insensitive, &features.transform) {
            (true, _) | (false, Some(TransformKind::Lowercase)) => repr.to_lowercase(),
            (false, Some(TransformKind::Uppercase)) => repr.to_uppercase(),
            (false, None) => repr,
        };

        let lit: syn::Lit = syn::LitStr::new(&repr, ident.span()).into();

        quote! {
            #lit => Ok(#type_name::#ident),
        }
    });

    let to_match = if features.case_insensitive {
        quote! { s.to_lowercase() }
    } else {
        quote! { s }
    };

    quote! {
        impl ::std::str::FromStr for #type_name {
            type Err = ::sternum::UnknownVariantError<#type_name>;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match &*#to_match {
                    #(#matches)*
                    _ => Err(::sternum::UnknownVariantError::new(s)),
                }
            }
        }
    }
}
