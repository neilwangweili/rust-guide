use std::collections::HashMap;
use learn_rust::day5::{coin::Coin, if_let::is_quarter, us_state::UsState,
                       learn_match::match_coins, match_options::plus_one,
                       restaurant::eat_at_restaurant};
use learn_rust::day5::build_map_from_two_vector::build_map_from_two_vector;
use learn_rust::day5::for_all_element::for_all_element;
use learn_rust::day5::learn_entry::learn_entry;
use learn_rust::day5::learn_map::init_map;
use learn_rust::day5::learn_vector::init_vector;
use learn_rust::day5::read_element_in_vector::read_element_in_vector;

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
fn should_add_to_wait_list_twice() {
    eat_at_restaurant();
}


#[test]
fn should_get_toast_correctly() {
    assert_eq!(learn_rust::day5::restaurant::front_of_house::serving::eat_at_restaurant("Wheat"), "Wheat");
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
    } else {
        panic!();
    }
    if let Some(i) = map.get("Yellow") {
        assert_eq!(*i, 50);
    } else {
        panic!();
    }
}

#[test]
fn should_get_a_map_from_two_vector() {
    let (key, value) = build_map_from_two_vector();
    let map: HashMap<_, _> = key.iter().zip(value.iter()).collect();
    if let Some(i) = map.get(&String::from("Blue")) {
        assert_eq!(**i, 10);
    } else {
        panic!();
    }
    if let Some(i) = map.get(&String::from("Yellow")) {
        assert_eq!(**i, 50);
    } else {
        panic!();
    }
}

#[test]
fn should_set_value_if_not_exist() {
    let map = learn_entry();
    assert_eq!(map.get("Blue"), Some(&10));
    assert_eq!(map.get("Yellow"), Some(&50));
}
