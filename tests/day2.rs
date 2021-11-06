use learn_rust::day2::ownership_1::ownership_1;
use learn_rust::day2::ownership_2::ownership_2;
use learn_rust::day2::string_class::string_class;

#[test]
fn should_variable_in_function_block() {
    assert_eq!(ownership_1(), "");
}

#[test]
fn should_string_class_append_string_successfully() {
    assert_eq!(string_class(), "hello world!");
}

#[test]
fn should_stack_variable_have_no_ownership() {
    assert_eq!(ownership_2(), 2);
}
