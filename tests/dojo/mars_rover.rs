use learn_rust::dojo::mars_rover::direction::Direction;
use learn_rust::dojo::mars_rover::mars_rover::MarsRover;
use learn_rust::dojo::mars_rover::turn_left::TurnLeft;

#[test]
fn should_init_a_mars_rover_with_x_max_10_y_max_15_x_3_y_3() {
    assert_eq!(MarsRover::put_on(10, 10, 3, 3, Direction::N)
                   .report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_turn_left_to_east_at_north() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::N);
    let mut vec = Vec::new();
    vec.push(TurnLeft::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing East.");
}
