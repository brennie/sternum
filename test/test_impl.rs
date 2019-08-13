// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Debug, Eq, PartialEq, Sternum)]
enum A {
    Foo,
    Bar,
    Baz,
}

#[derive(Debug, Eq, PartialEq, Sternum)]
#[sternum(scoped)]
enum B {
    Qux,
    Quux,
    Corge,
}

#[test]
fn impl_display() {
    assert_eq!(A::Foo.to_string(), "Foo");
    assert_eq!(A::Bar.to_string(), "Bar");
    assert_eq!(A::Baz.to_string(), "Baz");
}

#[test]
fn impl_from_str() {
    assert_eq!(str::parse::<A>("Foo"), Ok(A::Foo));
    assert_eq!(str::parse::<A>("Bar"), Ok(A::Bar));
    assert_eq!(str::parse::<A>("Baz"), Ok(A::Baz));

    assert_eq!(
        str::parse::<A>("unknown"),
        Err(ParseAError("unknown".into()))
    );
}

#[test]
fn round_trip() {
    assert_eq!(str::parse::<A>(&A::Foo.to_string()), Ok(A::Foo));
    assert_eq!(str::parse::<A>(&A::Bar.to_string()), Ok(A::Bar));
    assert_eq!(str::parse::<A>(&A::Baz.to_string()), Ok(A::Baz));
}

#[test]
fn impl_display_scoped() {
    assert_eq!(B::Qux.to_string(), "B::Qux");
    assert_eq!(B::Quux.to_string(), "B::Quux");
    assert_eq!(B::Corge.to_string(), "B::Corge");
}

#[test]
fn impl_from_str_scoped() {
    assert_eq!(str::parse::<B>("B::Qux"), Ok(B::Qux));
    assert_eq!(str::parse::<B>("B::Quux"), Ok(B::Quux));
    assert_eq!(str::parse::<B>("B::Corge"), Ok(B::Corge));

    assert_eq!(str::parse::<B>("Qux"), Err(ParseBError("Qux".into())));
    assert_eq!(str::parse::<B>("Quux"), Err(ParseBError("Quux".into())));
    assert_eq!(str::parse::<B>("Corge"), Err(ParseBError("Corge".into())));
}

#[test]
fn round_trip_scoped() {
    assert_eq!(str::parse::<B>(&B::Qux.to_string()), Ok(B::Qux));
    assert_eq!(str::parse::<B>(&B::Quux.to_string()), Ok(B::Quux));
    assert_eq!(str::parse::<B>(&B::Corge.to_string()), Ok(B::Corge));
}
