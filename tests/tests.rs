// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

mod test_impl_display;

use trybuild::TestCases;

#[test]
fn compile_tests() {
    let t = TestCases::new();

    t.pass("tests/compile/just-derive.rs");
    t.compile_fail("tests/compile/require-enum.rs");
    t.compile_fail("tests/compile/require-unit-enum.rs");
    t.pass("tests/compile/redefine-std-fmt-display.rs");
}
