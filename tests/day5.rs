use rust_guide::day5::build_map_from_two_vector::build_map_from_two_vector;
use rust_guide::day5::calculate_number_of_letter_occurrences::calculate_number_of_letter_occurrences;
use rust_guide::day5::for_all_element::for_all_element;
use rust_guide::day5::learn_entry::learn_entry;
use rust_guide::day5::learn_map::init_map;
use rust_guide::day5::learn_vector::init_vector;
use rust_guide::day5::read_element_in_vector::read_element_in_vector;
use rust_guide::day5::{
    coin::Coin, if_let::is_quarter, learn_match::match_coins, match_options::plus_one,
    us_state::UsState,
};
use std::collections::HashMap;

#[test]
fn should_match_denomination_of_penny() {
    assert_eq!(match_coins(Coin::Penny), 1);
}

#[test]
fn should_match_denomination_of_nickel() {
    assert_eq!(match_coins(Coin::Nickel), 5);
}

#[test]
fn should_match_denomination_of_dime() {
    assert_eq!(match_coins(Coin::Dime), 10);
}

#[test]
fn should_match_denomination_of_quarter() {
    assert_eq!(match_coins(Coin::Quarter), 25);
}

#[test]
fn should_match_denomination_of_any_state() {
    assert_eq!(match_coins(Coin::AnotherEnumEg(UsState::Alabama)), 0);
}

#[test]
fn should_plus_one_in_option() {
    assert_eq!(plus_one(Some(5)), Some(6));
}

#[test]
fn should_none_plus_one_return_none() {
    assert_eq!(plus_one(None), None);
}

#[test]
fn is_a_quarter() {
    assert_eq!(is_quarter(Coin::Quarter), true);
}

#[test]
fn is_not_a_quarter() {
    assert_eq!(is_quarter(Coin::Dime), false);
}

#[test]
fn should_init_vector_correctly() {
    let (a, b) = init_vector();
    assert_eq!(a, vec![1, 2, 3]);
    assert_eq!(b, vec![1, 2, 3]);
}

#[test]
fn should_get_element_in_vector() {
    let a = read_element_in_vector()[2];
    assert_eq!(a, 3);
}

#[test]
fn should_for_all_element_in_vector() {
    assert_eq!(for_all_element(vec![1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn should_get_a_map_with_blue_10_yellow_50() {
    let map = init_map();
    if let Some(i) = map.get("Blue") {
        assert_eq!(*i, 10);
    }
    if let Some(i) = map.get("Yellow") {
        assert_eq!(*i, 50);
    }
}

#[test]
fn should_get_a_map_from_two_vector() {
    let (key, value) = build_map_from_two_vector();
    let map: HashMap<_, _> = key.iter().zip(value.iter()).collect();
    if let Some(i) = map.get(&String::from("Blue")) {
        assert_eq!(**i, 10);
    }
    if let Some(i) = map.get(&String::from("Yellow")) {
        assert_eq!(**i, 50);
    }
}

#[test]
fn should_set_value_if_not_exist() {
    let map = learn_entry();
    assert_eq!(map.get("Blue"), Some(&10));
    assert_eq!(map.get("Yellow"), Some(&50));
}

#[test]
fn should_calculate_letter_numbers() {
    let result = calculate_number_of_letter_occurrences("Hello rust! Rust is powerful.");
    assert_eq!(result.get(&'e'), Some(&2));
    assert_eq!(result.get(&'r'), Some(&3));
    assert_eq!(result.get(&'s'), Some(&3));
    assert_eq!(result.get(&'i'), Some(&1));
    assert_eq!(result.get(&'f'), Some(&1));
    assert_eq!(result.get(&'w'), Some(&1));
    assert_eq!(result.get(&'h'), Some(&1));
    assert_eq!(result.get(&'u'), Some(&3));
    assert_eq!(result.get(&'t'), Some(&2));
    assert_eq!(result.get(&'e'), Some(&2));
    assert_eq!(result.get(&'o'), Some(&2));
    assert_eq!(result.get(&'p'), Some(&1));
}
