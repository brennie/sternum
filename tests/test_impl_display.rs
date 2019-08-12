// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Sternum)]
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
