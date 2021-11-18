use std::io::Error;
use learn_rust::day6::direct_panic::direct_panic;
use learn_rust::day6::read_username_from_file::read_username_from_file;
use learn_rust::day6::recoverable_panic::recoverable_panic;
use learn_rust::day6::unimplemented::unimplemented;
use learn_rust::day6::unreachable::unreachable;

#[test]
#[should_panic(expected = "Crash and burn!")]
pub fn should_panic_directly() {
    direct_panic();
}

#[test]
pub fn should_recoverable_panic_be_caught() {
    assert_eq!(recoverable_panic(), "Error");
}

#[test]
#[should_panic(expected = "No such file or directory")]
pub fn should_panic_directly_with_question_mark() {
    let a = read_username_from_file();
    match a {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}

#[test]
#[should_panic(expected = "not implemented")]
pub fn unimplemented_method() {
    unimplemented();
}

#[test]
#[should_panic(expected = "")]
pub fn unreachable_method() {
    unreachable();
}
