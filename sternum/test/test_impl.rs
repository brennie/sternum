// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::{Sternum, UnknownVariantError};

#[derive(Debug, Eq, PartialEq, Sternum)]
enum Enum {
    Foo,
    Bar,
    Baz,
}

#[test]
fn impl_display() {
    assert_eq!(Enum::Foo.to_string(), "Foo");
    assert_eq!(Enum::Bar.to_string(), "Bar");
    assert_eq!(Enum::Baz.to_string(), "Baz");
}

#[test]
fn impl_from_str() {
    assert_eq!(str::parse::<Enum>("Foo"), Ok(Enum::Foo));
    assert_eq!(str::parse::<Enum>("Bar"), Ok(Enum::Bar));
    assert_eq!(str::parse::<Enum>("Baz"), Ok(Enum::Baz));

    assert_eq!(
        str::parse::<Enum>("unknown"),
        Err(UnknownVariantError::new("unknown")),
    );
}

#[test]
fn round_trip() {
    assert_eq!(str::parse::<Enum>(&Enum::Foo.to_string()), Ok(Enum::Foo));
    assert_eq!(str::parse::<Enum>(&Enum::Bar.to_string()), Ok(Enum::Bar));
    assert_eq!(str::parse::<Enum>(&Enum::Baz.to_string()), Ok(Enum::Baz));
}
