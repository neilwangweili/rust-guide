use learn_rust::dojo::mars_rover::direction::Direction;
use learn_rust::dojo::mars_rover::mars_rover::MarsRover;

#[test]
fn should_init_a_mars_rover_with_x_max_10_y_max_15_x_3_y_3() {
    assert_eq!(MarsRover::put_on(10, 10, 3, 3, Direction::N)
                   .report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}
