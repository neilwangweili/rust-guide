use learn_rust::day6::direct_panic::direct_panic;

#[test]
#[should_panic(expected = "Crash and burn!")]
pub fn should_panic_directly() {
    direct_panic();
}
