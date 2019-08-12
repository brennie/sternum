// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use trybuild::TestCases;

#[test]
fn compile_tests() {
    let t = TestCases::new();

    t.pass("src/tests/compile/just-derive.rs");
}
