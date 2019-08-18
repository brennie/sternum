// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error, Ident, Token};

/// A raw feature, parseable from a [`TokenStream`][TokenStream].
///
/// [TokenStream]: ../proc-macro2/struct.TokenStream.html
#[derive(Debug, Eq, PartialEq)]
pub(super) enum RawFeature {
    Scoped { ident: Ident },
}

/// The comma-separated list of tokens that make up the arguments to the `#[sternum(...)]`
/// attribute.
///
/// It is parseable direcly from a [`TokenStream`][TokenStream].
///
/// [TokenStream]: ../proc-macro2/struct.TokenStream.html
#[derive(Debug, Eq, PartialEq)]
pub(super) struct RawFeatures {
    pub features: Punctuated<RawFeature, Token![,]>,
}

impl ToTokens for RawFeature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use RawFeature::*;

        match self {
            Scoped { ref ident } => ident.to_tokens(tokens),
        }
    }
}

impl Parse for RawFeature {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        use RawFeature::*;

        let ident: Ident = input.parse()?;
        let ident_name = ident.to_string();

        let feature = match &*ident_name {
            "scoped" => Scoped { ident },

            _ => {
                return Err(Error::new_spanned(
                    ident,
                    format!("Unknown argument `{}' for #[sternum] attribute", ident_name),
                ));
            }
        };

        Ok(feature)
    }
}

impl Parse for RawFeatures {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        Ok(RawFeatures {
            features: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}
