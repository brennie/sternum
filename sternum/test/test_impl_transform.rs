// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::{Sternum, UnknownVariantError};

#[derive(Debug, Eq, PartialEq, Sternum)]
#[sternum(transform = uppercase)]
enum UppercaseEnum {
    Foo,
    Bar,
    Baz,
}

#[derive(Debug, Eq, PartialEq, Sternum)]
#[sternum(transform = lowercase)]
enum LowercaseEnum {
    Foo,
    Bar,
    Baz,
}

#[test]
fn impl_display() {
    assert_eq!(UppercaseEnum::Foo.to_string(), "FOO");
    assert_eq!(UppercaseEnum::Bar.to_string(), "BAR");
    assert_eq!(UppercaseEnum::Baz.to_string(), "BAZ");

    assert_eq!(LowercaseEnum::Foo.to_string(), "foo");
    assert_eq!(LowercaseEnum::Bar.to_string(), "bar");
    assert_eq!(LowercaseEnum::Baz.to_string(), "baz");
}

#[test]
fn impl_from_str() {
    assert_eq!(str::parse::<UppercaseEnum>("FOO"), Ok(UppercaseEnum::Foo));
    assert_eq!(str::parse::<UppercaseEnum>("BAR"), Ok(UppercaseEnum::Bar));
    assert_eq!(str::parse::<UppercaseEnum>("BAZ"), Ok(UppercaseEnum::Baz));

    assert_eq!(str::parse::<LowercaseEnum>("foo"), Ok(LowercaseEnum::Foo));
    assert_eq!(str::parse::<LowercaseEnum>("bar"), Ok(LowercaseEnum::Bar));
    assert_eq!(str::parse::<LowercaseEnum>("baz"), Ok(LowercaseEnum::Baz));

    assert_eq!(
        str::parse::<UppercaseEnum>("Foo"),
        Err(UnknownVariantError::new("Foo"))
    );
    assert_eq!(
        str::parse::<UppercaseEnum>("Bar"),
        Err(UnknownVariantError::new("Bar"))
    );
    assert_eq!(
        str::parse::<UppercaseEnum>("Baz"),
        Err(UnknownVariantError::new("Baz"))
    );

    assert_eq!(
        str::parse::<LowercaseEnum>("Foo"),
        Err(UnknownVariantError::new("Foo"))
    );
    assert_eq!(
        str::parse::<LowercaseEnum>("Bar"),
        Err(UnknownVariantError::new("Bar"))
    );
    assert_eq!(
        str::parse::<LowercaseEnum>("Baz"),
        Err(UnknownVariantError::new("Baz"))
    );

}

#[test]
fn round_trip() {
    assert_eq!(str::parse::<UppercaseEnum>(&UppercaseEnum::Foo.to_string()), Ok(UppercaseEnum::Foo));
    assert_eq!(str::parse::<UppercaseEnum>(&UppercaseEnum::Bar.to_string()), Ok(UppercaseEnum::Bar));
    assert_eq!(str::parse::<UppercaseEnum>(&UppercaseEnum::Baz.to_string()), Ok(UppercaseEnum::Baz));

    assert_eq!(str::parse::<LowercaseEnum>(&LowercaseEnum::Foo.to_string()), Ok(LowercaseEnum::Foo));
    assert_eq!(str::parse::<LowercaseEnum>(&LowercaseEnum::Bar.to_string()), Ok(LowercaseEnum::Bar));
    assert_eq!(str::parse::<LowercaseEnum>(&LowercaseEnum::Baz.to_string()), Ok(LowercaseEnum::Baz));
}
