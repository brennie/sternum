// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error;
use std::fmt;
use std::marker::PhantomData;

pub use sternum_derive::Sternum;

#[derive(Eq, PartialEq)]
pub struct UnknownVariantError<T> {
    pub variant: String,
    _ty: PhantomData<T>,
}

impl<T> UnknownVariantError<T> {
    pub fn new(variant: &str) -> Self {
        UnknownVariantError {
            variant: variant.into(),
            _ty: PhantomData,
        }
    }
}

pub trait Sternum {
    fn type_name() -> &'static str;
}

impl<T> fmt::Debug for UnknownVariantError<T>
where
    T: Sternum,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&format!(
            "UnknownVariantError<{}>",
            <T as Sternum>::type_name(),
        ))
        .field("variant", &self.variant)
        .finish()
    }
}

impl<T> fmt::Display for UnknownVariantError<T>
where
    T: Sternum,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not parse `{}' as type {}: unknown variant",
            self.variant,
            <T as Sternum>::type_name()
        )
    }
}

impl<T> error::Error for UnknownVariantError<T> where T: Sternum {}
