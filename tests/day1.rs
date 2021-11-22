use rust_guide::day1::break_with_numbers::break_with_numbers;
use rust_guide::day1::break_with_tags::break_with_tags;
use rust_guide::day1::const_shadowing::const_shadowing;
use rust_guide::day1::demo1hello_world::hello_world;
use rust_guide::day1::demo2compare_number::compare_number;
use rust_guide::day1::demo3type_change::type_change;
use rust_guide::day1::demo4guess_number::guess_number;
use rust_guide::day1::demo5variable_readable::variable_readable;
use rust_guide::day1::demo6data_type::data_type;
use rust_guide::day1::for_loop::for_loop;
use rust_guide::day1::given_value_block::given_value_block;
use rust_guide::day1::learn_if::learn_if;
use rust_guide::day1::while_loop::while_loop;

#[test]
fn hello_world_should_return_hello_world() {
    assert_eq!(hello_world(), "Hello world");
}

#[test]
fn should_guess_number_5_return_true() {
    assert_eq!(compare_number(5), true);
}

#[test]
fn should_guess_number_3_return_false() {
    assert_eq!(compare_number(3), false);
}

#[test]
fn should_guess_number_10_return_false() {
    assert_eq!(compare_number(10), false);
}

#[test]
fn should_change_type_from_string_to_int() {
    assert_eq!(type_change("5"), 5);
}

#[test]
#[should_panic(expected = "Fail to convert to number.")]
fn should_not_change_type_from_illegal_string_to_int() {
    type_change("a");
}

#[test]
fn should_guess_number_match_secret_number() {
    assert_eq!(guess_number(), true);
}

#[test]
fn should_variable_with_mut_tag_edit_its_value() {
    assert_eq!(variable_readable(), 6);
}

#[test]
fn should_shadowing_variable() {
    assert_eq!(const_shadowing(5), 6);
}

#[test]
fn show_data_types() {
    data_type();
}

#[test]
fn should_given_value_block_return_correctly() {
    assert_eq!(given_value_block(), 6);
}

#[test]
fn should_return_true_if_x_more_than_zero() {
    assert_eq!(learn_if(5), true);
}

#[test]
fn should_return_true_if_x_equal_zero() {
    assert_eq!(learn_if(0), true);
}

#[test]
fn should_return_false_if_x_less_than_zero() {
    assert_eq!(learn_if(-5), false);
}

#[test]
fn should_return_3_in_double_loop() {
    assert_eq!(break_with_tags(), 3);
}

#[test]
fn should_return_20_in_loop_return() {
    assert_eq!(break_with_numbers(3), 20);
}

#[test]
fn should_while_loop_run_correctly() {
    assert_eq!(while_loop(), 0);
}

#[test]
fn should_for_run_correctly() {
    assert_eq!(for_loop(), 50);
}

