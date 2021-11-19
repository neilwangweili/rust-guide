use learn_rust::day6_dojo_scaffold::fizz_buzz::fizz_buzz::FizzBuzz;

#[test]
fn should_1_return_1() {
    assert_eq!(FizzBuzz::new().report(1), "1");
}
