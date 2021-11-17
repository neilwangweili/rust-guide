use learn_rust::day5::coin::Coin;
use learn_rust::day5::us_state::UsState;
use learn_rust::day5::learn_match::match_coins;

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
