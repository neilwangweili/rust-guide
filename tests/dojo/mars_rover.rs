use learn_rust::dojo::mars_rover::command::Command;
use learn_rust::dojo::mars_rover::direction::Direction;
use learn_rust::dojo::mars_rover::mars_rover::MarsRover;
use learn_rust::dojo::mars_rover::run_toward::RunToward;
use learn_rust::dojo::mars_rover::turn_left::TurnLeft;
use learn_rust::dojo::mars_rover::turn_right::TurnRight;

#[test]
fn should_init_a_mars_rover_with_x_max_10_y_max_15_x_3_y_3() {
    assert_eq!(generate_mars_rover_for_test().report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_turn_left_to_east_at_north() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnLeft::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing East.");
}

#[test]
fn should_mars_rover_turn_left_to_south_at_east() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnLeft::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_turn_left_to_west_at_south() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnLeft::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_turn_left_to_north_at_west() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnLeft::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_turn_right_to_west_at_north() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnRight::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_turn_right_to_south_at_west() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnRight::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_turn_right_to_east_at_south() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnRight::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing East.");
}

#[test]
fn should_mars_rover_turn_right_to_north_at_east() {
    test_template(&mut generate_mars_rover_for_test(), &vec![TurnRight::new()], "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_move_toward_at_east() {
    test_template(&mut generate_mars_rover_for_test(), &vec![RunToward::new()], "I'm 4 on the X-axis and 3 on the Y-axis and facing East.");
}

#[test]
fn should_mars_rover_move_toward_at_south() {
    test_template(&mut generate_mars_rover_for_test(), &vec![RunToward::new()], "I'm 3 on the X-axis and 4 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_move_toward_at_west() {
    test_template(&mut generate_mars_rover_for_test(), &vec![RunToward::new()], "I'm 2 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_move_toward_at_north() {
    test_template(&mut generate_mars_rover_for_test(), &vec![RunToward::new()], "I'm 3 on the X-axis and 2 on the Y-axis and facing North.");
}

fn generate_mars_rover_for_test() -> MarsRover {
    MarsRover::put_on(10, 10, 3, 3, Direction::N)
}

fn test_template<T: Command>(mars_rover: &mut MarsRover, commands: &Vec<T>, valid_string: &str) {
    mars_rover.execute_commands(commands);
    assert_eq!(mars_rover.report(), valid_string);
}
