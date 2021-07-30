#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/contract/01-flipper-contract.rs");
    t.pass("tests/contract/02-flipper-owner-contract.rs");
    t.pass("tests/contract/03-stub.rs");
    // t.pass("tests/02-create-builder.rs");
    // t.pass("tests/03-call-setters.rs");
    // t.pass("tests/04-call-build.rs");
    // t.pass("tests/05-method-chaining.rs");
    // t.pass("tests/06-optional-field.rs");
    // t.pass("tests/07-repeated-field.rs");
    // t.compile_fail("tests/08-unrecognized-attribute.rs");
    // t.pass("tests/09-redefined-prelude-types.rs");
}
