// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Sternum)]
struct A;

#[derive(Sternum)]
struct B {}

#[derive(Sternum)]
struct C {
    a: i32,
}

#[derive(Sternum)]
union D {
    a: i32,
}

fn main() {}
