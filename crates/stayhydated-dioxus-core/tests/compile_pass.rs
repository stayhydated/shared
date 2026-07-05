#[test]
fn component_api_compile_passes() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/pass/*.rs");
}
