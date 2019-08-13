// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
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

#[derive(Debug, Default, Eq, PartialEq)]
struct Features {
    scoped: bool,
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

    let features = parse_features(&ast.attrs)?;

    let display_impl = impl_display(&ast.ident, variants.iter(), &features);
    let from_str_impl = impl_from_str(&ast.ident, variants.iter(), &ast.vis, &features);

    let quoted = quote! {
        #display_impl
        #from_str_impl
    };

    Ok(quoted.into())
}

fn impl_display<'a, I>(
    name: &syn::Ident,
    variants: I,
    features: &Features,
) -> proc_macro2::TokenStream
where
    I: Iterator<Item = &'a syn::Variant>,
{
    let matches = variants.map(|variant| {
        let ident = &variant.ident;
        let str_repr = if features.scoped {
            format!("{}::{}", name, ident)
        } else {
            ident.to_string()
        };

        let repr: syn::Lit = syn::LitStr::new(&str_repr, ident.span()).into();

        quote! {
            #name::#ident => write!(f, "{}", #repr),
        }
    });

    quote! {
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    #(#matches)*

                }
            }
        }
    }
}

fn impl_from_str<'a, I>(
    name: &syn::Ident,
    variants: I,
    visibility: &syn::Visibility,
    features: &Features,
) -> proc_macro2::TokenStream
where
    I: Iterator<Item = &'a syn::Variant>,
{
    let matches = variants.map(|variant| {
        let ident = &variant.ident;
        let repr = if features.scoped {
            format!("{}::{}", name, ident)
        } else {
            ident.to_string()
        };

        let lit: syn::Lit = syn::LitStr::new(&repr, ident.span()).into();

        quote! {
            #lit => Ok(#name::#ident),
        }
    });

    let error_ident = syn::Ident::new(&format!("Parse{}Error", name), name.span());

    quote! {
        #[derive(Debug, Eq, PartialEq)]
        #visibility struct #error_ident (pub String);

        impl ::std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "Could not parse `{}': unknown ident", self.0)
            }
        }

        impl ::std::error::Error for #error_ident {}

        impl ::std::str::FromStr for #name {
            type Err = #error_ident;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#matches)*
                    _ => Err(#error_ident(s.into())),
                }
            }
        }
    }
}

fn parse_features(attrs: &[syn::Attribute]) -> Result<Features, ErrorList> {
    let mut errors = vec![];
    let mut features = Features::default();

    for attr in attrs {
        let list = match get_meta_list(&attr) {
            Ok(Some(list)) => list,
            Ok(None) => continue,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        for item in list.nested.iter() {
            match item {
                syn::NestedMeta::Meta(syn::Meta::Word(ref ident)) => match &*ident.to_string() {
                    "scoped" => features.scoped = true,
                    unknown => errors.push(Error::new_spanned(
                        ident,
                        format!("Unexpected attribute `#[sternum({})]'", unknown),
                    )),
                },

                _ => errors.push(Error::new_spanned(
                    item,
                    format!("Unexpected attribute `#[sternum({})]'", quote! { #item },),
                )),
            }
        }
    }

    if errors.len() == 0 {
        Ok(features)
    } else {
        Err(ErrorList(errors))
    }
}

fn get_meta_list(attr: &syn::Attribute) -> Result<Option<syn::MetaList>, Error> {
    let meta = attr.parse_meta()?;

    if meta.name() != "sternum" {
        return Ok(None);
    }

    match meta {
        syn::Meta::Word(..) => Err(Error::new_spanned(
            meta,
            "Unexpected attribute #[sternum]; expected #[sternum(...)]",
        )),
        syn::Meta::NameValue(..) => Err(Error::new_spanned(
            meta,
            "Unexpected attribute #[sternum = ...]; expected #[sternum(...)]",
        )),
        syn::Meta::List(meta_list) => Ok(Some(meta_list)),
    }
}
