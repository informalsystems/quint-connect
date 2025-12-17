#[test]
fn test_macros() {
    let t = trybuild::TestCases::new();
    t.pass("tests/macros/*/pass/*.rs");
    t.compile_fail("tests/macros/*/fail/*.rs");
}

#[test]
#[cfg(not(nightly))]
fn test_macros_stable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macros/*/fail/stable/*.rs");
}

#[test]
#[cfg(nightly)]
fn test_macros_nightly() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macros/*/fail/nightly/*.rs");
}
