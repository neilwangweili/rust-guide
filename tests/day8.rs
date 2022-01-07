use rust_guide::day8::add_four::add_four;
use rust_guide::day8::workout::workout;

#[test]
fn should_do_intensity_8_random_numbers_5_push_ups() {
    assert_eq!(workout(8, 5), "Today, do 8 push ups! Next, do 8 sit ups!");
}

#[test]
fn should_take_a_break_when_random_number_3() {
    assert_eq!(workout(28, 3), "Take a break today.");
}

#[test]
fn run_30_circles() {
    assert_eq!(workout(30, 0), "Today, run 30 circles.");
}

#[test]
fn should_add_four_correctly() {
    assert_eq!(add_four(1), 5);
}
