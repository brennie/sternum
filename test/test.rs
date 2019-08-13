// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

mod test_impl;

use trybuild::TestCases;

#[test]
fn compile() {
    let t = TestCases::new();

    t.pass("test/compile/just-derive.rs");
    t.compile_fail("test/compile/require-enum.rs");
    t.compile_fail("test/compile/require-unit-enum.rs");
    t.pass("test/compile/redefine-std.rs");
    t.compile_fail("test/compile/require-enum-variants.rs");
    t.compile_fail("test/compile/invalid-attributes.rs");
}
