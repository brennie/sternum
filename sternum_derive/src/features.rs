// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod parse;

use std::convert::TryFrom;

use either::Either;
use syn::Error;

use crate::error::ErrorList;
use crate::features::parse::{RawFeature, RawFeatures};

/// The set of features that the Sternum derive should use.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct FeatureSet {
    pub scoped: bool,
    pub transform: Option<TransformKind>,
}

/// An uppercase or lowercase transform.
#[derive(Debug, Eq, PartialEq)]
pub enum TransformKind {
    Uppercase,
    Lowercase,
}

/// A singular feature that effects the behaviour of Sternum.
#[derive(Debug, Eq, PartialEq)]
struct Feature {
    /// The kind of feature.
    kind: FeatureKind,

    /// The raw tokens for this feature.
    ///
    /// This is used for reporting errors.
    raw: RawFeature,
}

impl TryFrom<RawFeature> for Feature {
    type Error = Error;

    fn try_from(raw: RawFeature) -> Result<Self, Self::Error> {
        let kind = match raw {
            RawFeature::Scoped { .. } => FeatureKind::Scoped,
            RawFeature::Transform { ref value, .. } => {
                let trans = match &*value.to_string() {
                    "uppercase" => TransformKind::Uppercase,
                    "lowercase" => TransformKind::Lowercase,
                    _ => return Err(Error::new_spanned(
                        value,
                        "Unexpected value for #[sternum(transform = ...)]; expected `uppercase' or `lowercase'")),
                };

                FeatureKind::Transform(trans)
            }
        };

        Ok(Feature {
            kind,
            raw,
        })
    }
}

/// The kind of feature.
///
/// This is the actual data structure that is matched on to compute a [`FeatureSet`][FeatureSet].
///
/// [FeatureSet]: struct.FeatureSet.html
#[derive(Debug, Eq, PartialEq)]
enum FeatureKind {
    Scoped,
    Transform(TransformKind),
}

impl FeatureSet {
    /// Attempt to apply the feature to the `FeatureSet`.
    ///
    /// An error indicates that the provided feature conflicts with the current set of features.
    fn apply(&mut self, f: Feature) -> Result<(), Error> {
        use FeatureKind::*;

        match f.kind {
            Scoped => {
                self.scoped = true;
            }

            Transform(trans) => match &self.transform {
                Some(prev_trans) => {
                    if *prev_trans != trans {
                        return Err(Error::new_spanned(f.raw, "Repeated"));
                    }
                }

                None => self.transform = Some(trans),
            }
        }

        Ok(())
    }
}

/// The current parsing state over the iterator of `Feature`s in `parse_features`.
#[derive(Debug, Default)]
struct ParseState {
    /// The accumulated errors, either from earlier in parsing or from calling
    /// [`FeatureSet::apply`][FeatureSet::apply] on incoming [`Feature`s][Feature].
    ///
    /// [Feature]: struct.Feature.html
    /// [FeatureSet::apply]: struct.FeatureSet.html#method.apply
    errors: Vec<Error>,
    features: FeatureSet,
}

impl ParseState {
    /// Finalize the ParseState into a set of Features (if we have no errors) or the accumulated
    /// errors.
    fn finalize(self) -> Result<FeatureSet, ErrorList> {
        if self.errors.len() == 0 {
            Ok(self.features)
        } else {
            Err(ErrorList(self.errors))
        }
    }
}

/// Attempt to parse the arguments to all `#[sternum(...)]` attributes into a
/// [`FeatureSet`][FeatureSet].
///
/// [FeatureSet]: struct.FeatureSet.html
pub fn parse_features(attrs: &[syn::Attribute]) -> Result<FeatureSet, ErrorList> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("sternum"))
        .map(|attr| attr.parse_args::<RawFeatures>())
        .flat_map(|result| {
            // For any `RawFeatures` instances we get from attributes, transform it into an
            // iterator over its `RawFeature`s and attempt to convert it to `Feature`. This will
            // wrap it in a result, allowing us to carry along errors form the previous step.
            match result {
                Ok(fs) => Either::Left(fs.features.into_iter().map(Feature::try_from)),
                Err(e) => Either::Right(std::iter::once(Err(e))),
            }
        })
        .fold(ParseState::default(), |mut state, item| {
            // We don't .collect() here so that we can return as much error information as possible
            // to the user.
            match item {
                // If we find a feature, attempt to apply it to the current feature set, finding
                // feature conflicts and reporting them as errors.
                Ok(feature) => {
                    if let Err(e) = state.features.apply(feature) {
                        state.errors.push(e);
                    }
                }

                // Otherwise, collect the errors.
                Err(e) => state.errors.push(e),
            }

            state
        })
        .finalize()
}
