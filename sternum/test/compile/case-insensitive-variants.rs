// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use sternum::Sternum;

#[derive(Sternum)]
enum A {
    Variant,
    VARIANT,
    VarIaNT,
}


#[derive(Sternum)]
#[sternum(case_insensitive)]
enum B {
    Variant,
    VARIANT,
    VarIaNT,
}

#[derive(Sternum)]
#[sternum(transform = uppercase)]
enum C {
    Variant,
    VARIANT,
    VarIaNT,
}

#[derive(Sternum)]
#[sternum(transform = lowercase)]
enum D {
    Variant,
    VARIANT,
    VarIaNT,
}

fn main() {}
