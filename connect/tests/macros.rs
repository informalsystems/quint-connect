#[test]
fn test_macros() {
    let t = trybuild::TestCases::new();
    t.pass("tests/macros/*/pass/*.rs");
    t.compile_fail("tests/macros/*/fail/*.rs");
}
