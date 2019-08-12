// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Debug, Eq, PartialEq, Sternum)]
enum A {
    Foo,
    Bar,
    Baz,
}

#[test]
fn impl_display() {
    assert_eq!(format!("{}", A::Foo), "Foo");
    assert_eq!(format!("{}", A::Bar), "Bar");
    assert_eq!(format!("{}", A::Baz), "Baz");
}

#[test]
fn impl_from_str() {
    assert_eq!(str::parse::<A>("Foo"), Ok(A::Foo));
    assert_eq!(str::parse::<A>("Bar"), Ok(A::Bar));
    assert_eq!(str::parse::<A>("Baz"), Ok(A::Baz));

    assert_eq!(str::parse::<A>("unknown"), Err(ParseAError("unknown".into())));
}
