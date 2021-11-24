use rust_guide::{add, calculate_result, create_function};

#[test]
fn should_create_function_macro_rule_run_correctly() {
    create_function!(foo);
    assert_eq!(foo(), "You called \"foo\"()");
}

#[test]
fn should_calculate_result_macro_rule_run_correctly() {
    assert_eq!(calculate_result!(5 + 6), 11);
}

#[test]
fn should_add_macro_rule_run_correctly() {
    assert_eq!(add!(1, 2), 3);
}
