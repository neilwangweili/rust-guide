use rust_guide::day7::complex::Complex;
use rust_guide::{add, bar, calculate_result, create_function, find_min, hash, vector};
use std::collections::HashMap;

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

#[test]
fn should_bar_create_a_field_3() {
    bar!(a);
    assert_eq!(a, 3);
}

#[test]
fn should_create_map_like_ruby() {
    let map = hash!(1 => 2, 2 => 3, 3 => 4);
    assert_eq!(map.get(&1).unwrap(), &2);
    assert_eq!(map.get(&2).unwrap(), &3);
    assert_eq!(map.get(&3).unwrap(), &4);
}

#[test]
fn should_rewrite_add() {
    let z = Complex::new(2.0, 3.0) + Complex::new(4.0, 6.0);
    assert_eq!(z.a(), 6.0);
    assert_eq!(z.b(), 9.0);
}
