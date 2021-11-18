use learn_rust::day6::direct_panic::direct_panic;
use learn_rust::day6::recoverable_panic::recoverable_panic;

#[test]
#[should_panic(expected = "Crash and burn!")]
pub fn should_panic_directly() {
    direct_panic();
}

#[test]
pub fn should_recoverable_panic_be_caught() {
    assert_eq!(recoverable_panic(), "Error");
}
