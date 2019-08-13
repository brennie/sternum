// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Sternum)]
#[sternum]
enum A {
    A1,
}


#[derive(Sternum)]
#[sternum = "help"]
enum B {
    B1,
}

#[derive(Sternum)]
#[sternum = 123]
enum C {
    C1,
}

#[derive(Sternum)]
#[sternum(foo = "bar")]
enum D {
    D1,
}

#[derive(Sternum)]
#[sternum(unknown)]
enum E {
    E1,
}

fn main() {}
