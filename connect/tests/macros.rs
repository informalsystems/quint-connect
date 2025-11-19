#[test]
fn test_quint_run() {
    let t = trybuild::TestCases::new();
    t.pass("tests/macros/quint_run/pass/*.rs");
    t.compile_fail("tests/macros/quint_run/fail/*.rs");
}
