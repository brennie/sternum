// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Sternum
//!
//! Sternum is a derive macro for generarting Enum <-> String conversions. Specifically, it will
//! generate [`Display`][std::fmt::Display] and [`FromStr`][std::str::FromStr] implementations for
//! your enum.
//!
//!
//! # Example
//! ```
//! # use sternum::Sternum;
//! #[derive(Debug, Eq, PartialEq, Sternum)]
//! enum Kind {
//!     Foo,
//!     Bar,
//!     Baz
//! }
//!
//! // Display
//! assert_eq!(Kind::Foo.to_string(), "Foo");
//! assert_eq!(Kind::Bar.to_string(), "Bar");
//! assert_eq!(Kind::Baz.to_string(), "Baz");
//!
//! // FromStr
//! assert_eq!(str::parse::<Kind>("Foo"), Ok(Kind::Foo));
//! assert_eq!(str::parse::<Kind>("Bar"), Ok(Kind::Bar));
//! assert_eq!(str::parse::<Kind>("Baz"), Ok(Kind::Baz));
//! ```
//!
//! ## Attributes
//!
//! Sternum is customizable through the `#![sternum(...)]` attribute macro, which
//! supports the following:
//!
//! 1. Scoped Names
//!
//!    By default, the generated `Display` and `FromStr` implementations are unscoped. To support
//!    names scoped under their enumeration's name, the `#[sternum(scoped)]` attribute can be
//!    applied to the entire enum:
//!
//!    ```
//!    # use sternum::{Sternum, UnknownVariantError};
//!
//!    #[derive(Debug, Eq, PartialEq, Sternum)]
//!    #[sternum(scoped)]
//!    enum Enum {
//!        Variant,
//!    }
//!
//!    assert_eq!(Enum::Variant.to_string(), "Enum::Variant");
//!    assert_eq!(str::parse::<Enum>("Enum::Variant"), Ok(Enum::Variant));
//!
//!    assert_eq!(str::parse::<Enum>("Variant"), Err(UnknownVariantError::new("Variant")));
//!    ```
//!
//! ## `FromStr`
//!
//! Each `FromStr` implementation will use the
//! [`UnknownVariantError`][sternum::UnknownVariantError] type for
//! [`FromStr::Err`][std::str::FromStr::Err].
//!
//! ```
//! # use sternum::{Sternum, UnknownVariantError};
//!
//! #[derive(Debug, Eq, PartialEq, Sternum)]
//! enum Enum {
//!     Foo,
//! }
//!
//! assert_eq!(str::parse::<Enum>("unknown"), Err(UnknownVariantError::new("unknown")));
//!
//! ```
//!
//! [std::fmt::Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [std::str::FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [std::str::FromStr::Err]: https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err
//!
//! [sternum::UnknownVariantError]: struct.UnknownVariantError.html

use std::error;
use std::fmt;
use std::marker::PhantomData;

pub use sternum_derive::Sternum;

#[derive(Eq, PartialEq)]
/// An error indicating that a string could not be parsed as a `T` variant.
pub struct UnknownVariantError<T> {
    /// The string that could not be parsed.
    pub variant: String,
    _ty: PhantomData<T>,
}

impl<T> UnknownVariantError<T> {
    /// Generate a new error.
    pub fn new(variant: &str) -> Self {
        UnknownVariantError {
            variant: variant.into(),
            _ty: PhantomData,
        }
    }
}

/// The Sternum trait
pub trait Sternum {
    /// The name of the type.
    ///
    /// This is used inside the `Debug` and `Display` implementations of
    /// [`UnknownVariantError`][sternum::UnknownVariantError].
    ///
    /// [sternum::UnknownVariantError]: struct.UnknownVariantError.html
    fn type_name() -> &'static str;
}

impl<T> fmt::Debug for UnknownVariantError<T>
where
    T: Sternum,
{
    // We cannot derive Debug for UnknownVariantError<T> since T may not implement Debug, but we
    // don't actually need to debug print any T values.
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
