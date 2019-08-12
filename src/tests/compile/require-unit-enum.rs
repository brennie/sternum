// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Sternum)]
enum A {
    A1 {},
}

#[derive(Sternum)]
enum B {
    B1(u32),
}

#[derive(Sternum)]
enum C {
    C1 {
        a: u32,
    }
}

#[derive(Sternum)]
enum D {
    D1,
    D2,
    D3(u32),
    D4 {
        a: u32,
    },
}

fn main() {}
