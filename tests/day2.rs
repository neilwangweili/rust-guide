use learn_rust::day2::clone::clone;
use learn_rust::day2::deconstruction::deconstruction;
use learn_rust::day2::ownership_1::ownership_1;
use learn_rust::day2::ownership_2::ownership_2;
use learn_rust::day2::ownership_3::ownership_3;
use learn_rust::day2::ownership_4::ownership_4;
use learn_rust::day2::ownership_5::ownership_5;
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

#[test]
fn should_import_type_variable_move_to_newest_variable() {
    assert_eq!(ownership_3(), "Hello world!");
}

#[test]
fn should_clone_doesnt_move_ownership() {
    assert_eq!(clone(), "Hello world!");
}

#[test]
fn should_method_take_only_ownership_of_heap_variable() {
    assert_eq!(ownership_4(), 5);
}

#[test]
fn should_move_ownership_up_and_on_methods() {
    assert_eq!(ownership_5(), "Hello world!");
}

#[test]
fn deconstruction_is_allowed() {
    let (s1, size) = deconstruction();
    assert_eq!(s1, "Hello world!");
    assert_eq!(size, 12);
}
