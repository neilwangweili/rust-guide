use learn_rust::day5::{coin::Coin, if_let::is_quarter, us_state::UsState,
                       learn_match::match_coins, match_options::plus_one,
                       restaurant::eat_at_restaurant};

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
