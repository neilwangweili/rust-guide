use rust_guide::day2::clone::clone;
use rust_guide::day2::deconstruction::deconstruction;
use rust_guide::day2::ownership_1::ownership_1;
use rust_guide::day2::ownership_2::ownership_2;
use rust_guide::day2::ownership_3::ownership_3;
use rust_guide::day2::ownership_4::ownership_4;
use rust_guide::day2::ownership_5::ownership_5;
use rust_guide::day2::references::references;
use rust_guide::day2::references_2::references_2;
use rust_guide::day2::references_3::references_3;
use rust_guide::day2::references_4::references_4;
use rust_guide::day2::slice_1::slice_1;
use rust_guide::day2::slice_2::slice_2;
use rust_guide::day2::string_class::string_class;

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

#[test]
fn borrow_doesnt_move_ownership() {
    let (s1, s2) = references();
    assert_eq!(s1, "Hello world!");
    assert_eq!(s2, "Hello world!");
}

#[test]
fn borrow_cant_edit_value() {
    references_2();
}

#[test]
fn should_mut_borrow_edit_value() {
    assert_eq!(references_3(), "Hello world!");
}

#[test]
fn should_scope_end_to_create_mut_borrow() {
    assert_eq!(references_4(), "hello");
}

#[test]
fn should_return_word_end_index() {
    assert_eq!(slice_1(&String::from("Hello world")), 5);
}

#[test]
fn should_return_word_end_index_no() {
    assert_eq!(slice_1(&String::from("Hello")), 5);
}

#[test]
fn slice_role() {
    let s = String::from("Hello world!");
    let (s1, s2, s3, s4) = slice_2(&s);
    assert_eq!(s1, "ello");
    assert_eq!(s2, "Hello");
    assert_eq!(s3, "o world!");
    assert_eq!(s4, "Hello world!");
}
