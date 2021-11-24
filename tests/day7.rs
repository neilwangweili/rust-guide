use rust_guide::{add, calculate_result, create_function, find_min, vector};

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

#[test]
fn should_vector_macro_rule_run_correctly() {
    assert_eq!(vector![1, 2], vec![1, 2]);
}

#[test]
fn find_min_1() {
    assert_eq!(find_min!(1), 1);
}

#[test]
fn find_min_2() {
    assert_eq!(find_min!(1, 2), 1);
}

#[test]
fn find_min_3() {
    assert_eq!(find_min!(1, 2, 3, 4, 5), 1);
}
