use learn_rust::dojo_scaffold::fizz_buzz::fizz_buzz::FizzBuzz;

#[test]
fn should_1_return_1() {
    assert_eq!(FizzBuzz::new(100).report(1), "1");
}

#[test]
fn should_3_return_fizz() {
    assert_eq!(FizzBuzz::new(100).report(3), "Fizz");
}

#[test]
fn should_5_return_buzz() {
    assert_eq!(FizzBuzz::new(100).report(5), "Buzz");
}

#[test]
fn should_15_return_fizz_buzz() {
    assert_eq!(FizzBuzz::new(100).report(15), "FizzBuzz");
}

#[test]
fn should_13_return_fizz() {
    assert_eq!(FizzBuzz::new(100).report(13), "Fizz");
}

#[test]
fn should_52_return_buzz() {
    assert_eq!(FizzBuzz::new(100).report(50), "Buzz");
}

#[test]
fn should_30_return_fizz_buzz() {
    assert_eq!(FizzBuzz::new(100).report(30), "FizzBuzz");
}

#[test]
fn should_51_return_fizz_buzz() {
    assert_eq!(FizzBuzz::new(100).report(51), "FizzBuzz");
}

#[test]
fn should_53_return_fizz_buzz() {
    assert_eq!(FizzBuzz::new(100).report(53), "FizzBuzz");
}
