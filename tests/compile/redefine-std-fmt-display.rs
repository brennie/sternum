// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

// Redefine std::fmt::{Display, Result, Formatter} (and variants) to ensure the
// derive implementation uses fully-scoped names.

mod std {
    mod fmt {
        struct Display;
        type Result = ();

        trait Formatter {}
    }
}

mod fmt {
    struct Display;
    type Result = ();
    trait Formatter {}
}

struct Display;
type Result = ();
trait Formatter {}


#[derive(Sternum)]
enum A {
    Foo,
}

fn main() {}
