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

/// The custom derive for the [`Sternum`][sternum::Sternum] trait.
///
/// Deriving this trait will also derive [`Display`][std::fmt::Display] and
/// [`FromStr`][std::str::FromStr] implementations.
///
/// [sternum::Sternum]: ../sternum/trait.Sternum.html
/// [std::fmt::Display]: https://doc.rust-lang.org/std/std/trait.Display.html
/// [std::str::FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
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

fn impl_sternum(type_name: &syn::Ident) -> proc_macro2::TokenStream {
    let type_name_as_str = type_name.to_string();

    quote! {
        impl ::sternum::Sternum for #type_name {
            fn type_name() -> &'static str {
                return #type_name_as_str;
            }
        }

    }
}

fn impl_display<'a, I>(
    type_name: &syn::Ident,
    variants: I,
    features: &Features,
) -> proc_macro2::TokenStream
where
    I: Iterator<Item = &'a syn::Variant>,
{
    let matches = variants.map(|variant| {
        let ident = &variant.ident;
        let str_repr = if features.scoped {
            format!("{}::{}", type_name, ident)
        } else {
            ident.to_string()
        };

        let repr: syn::Lit = syn::LitStr::new(&str_repr, ident.span()).into();

        quote! {
            #type_name::#ident => write!(f, "{}", #repr),
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

fn impl_from_str<'a, I>(
    type_name: &syn::Ident,
    variants: I,
    features: &Features,
) -> proc_macro2::TokenStream
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

        let lit: syn::Lit = syn::LitStr::new(&repr, ident.span()).into();

        quote! {
            #lit => Ok(#type_name::#ident),
        }
    });

    quote! {
        impl ::std::str::FromStr for #type_name {
            type Err = ::sternum::UnknownVariantError<#type_name>;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#matches)*
                    _ => Err(::sternum::UnknownVariantError::new(s)),
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
                syn::NestedMeta::Meta(syn::Meta::Path(ref path)) => {
                    if path.is_ident("scoped") {
                        features.scoped = true;
                    } else {
                        errors.push(Error::new_spanned(
                            path,
                            format!("Unexpected attribute `#[sternum({})]'", quote! { #path }),
                        ));
                    }
                }

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

    if !meta.path().is_ident("sternum") {
        return Ok(None);
    }

    match meta {
        syn::Meta::Path(..) => Err(Error::new_spanned(
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
