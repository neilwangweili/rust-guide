use rust_guide::create_function;

#[test]
fn should_create_function_macro_rule_run_correctly() {
    create_function!(foo);
    assert_eq!(foo(), "You called \"foo\"()");
}
