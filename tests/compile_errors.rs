//! Test that the macro produces helpful compile errors.

#[test]
fn compile_fail_tests() {
    trybuild::TestCases::new().compile_fail("tests/compile_fail/*.rs");
}
