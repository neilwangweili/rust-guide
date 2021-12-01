use rust_guide::dojo::mars_rover_journey::command::Command;
use rust_guide::dojo::mars_rover_journey::direction::Direction;
use rust_guide::dojo::mars_rover_journey::mars_rover::MarsRover;
use rust_guide::dojo::mars_rover_journey::run_back::RunBack;
use rust_guide::dojo::mars_rover_journey::run_toward::RunToward;
use rust_guide::dojo::mars_rover_journey::turn_left::TurnLeft;
use rust_guide::dojo::mars_rover_journey::turn_right::TurnRight;

#[test]
fn should_init_a_mars_rover_with_x_max_10_y_max_15_x_3_y_3() {
    assert_eq!(
        MarsRover::put_on(10, 10, 3, 3, Direction::N).report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_mars_rover_turn_left_to_east_at_north() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::N);
    let mut vec = Vec::new();
    vec.push(TurnLeft::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing East."
    );
}

#[test]
fn should_mars_rover_turn_left_to_south_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(TurnLeft::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing South."
    );
}

#[test]
fn should_mars_rover_turn_left_to_west_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(TurnLeft::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing West."
    );
}

#[test]
fn should_mars_rover_turn_left_to_north_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(TurnLeft::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_mars_rover_turn_right_to_west_at_north() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::N);
    let mut vec = Vec::new();
    vec.push(TurnRight::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing West."
    );
}

#[test]
fn should_mars_rover_turn_right_to_south_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(TurnRight::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing South."
    );
}

#[test]
fn should_mars_rover_turn_right_to_east_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(TurnRight::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing East."
    );
}

#[test]
fn should_mars_rover_turn_right_to_north_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(TurnRight::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_mars_rover_move_toward_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(RunToward::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 4.0 on the X-axis and 3.0 on the Y-axis and facing East."
    );
}

#[test]
fn should_mars_rover_move_toward_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(RunToward::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 4.0 on the Y-axis and facing South."
    );
}

#[test]
fn should_mars_rover_move_toward_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(RunToward::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 2.0 on the X-axis and 3.0 on the Y-axis and facing West."
    );
}

#[test]
fn should_mars_rover_move_back_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(RunBack::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 2.0 on the X-axis and 3.0 on the Y-axis and facing East."
    );
}

#[test]
fn should_mars_rover_move_back_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(RunBack::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 2.0 on the Y-axis and facing South."
    );
}

#[test]
fn should_mars_rover_move_back_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(RunBack::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 4.0 on the X-axis and 3.0 on the Y-axis and facing West."
    );
}

#[test]
fn should_mars_rover_move_back_at_north() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::N);
    let mut vec = Vec::new();
    vec.push(RunBack::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 4.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_mars_rover_run_successfully() {
    let mut mars_rover = MarsRover::put_on(10, 15, 3, 3, Direction::N);
    let mut vec: Vec<Box<dyn Command>> = Vec::new();
    vec.push(RunToward::make());
    vec.push(RunToward::make());
    vec.push(TurnLeft::make());
    vec.push(RunToward::make());
    vec.push(TurnRight::make());
    vec.push(RunBack::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(RunBack::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 4.0 on the X-axis and 1.0 on the Y-axis and facing South."
    );
}

#[test]
fn should_mars_rover_turns_correctly() {
    let mut mars_rover = MarsRover::put_on(10, 15, 3, 3, Direction::N);
    let mut vec: Vec<Box<dyn Command>> = Vec::new();
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    vec.push(TurnLeft::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_mars_rover_turns_correctly_2() {
    let mut mars_rover = MarsRover::put_on(10, 15, 3, 3, Direction::N);
    let mut vec: Vec<Box<dyn Command>> = Vec::new();
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    vec.push(TurnRight::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 3.0 on the X-axis and 3.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_stop_move_outside_n() {
    let mut mars_rover = MarsRover::put_on(5, 5, 5, 0, Direction::N);
    let mut vec = Vec::new();
    vec.push(RunToward::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 5.0 on the X-axis and 0.0 on the Y-axis and facing North."
    );
}

#[test]
fn should_stop_move_outside_s() {
    let mut mars_rover = MarsRover::put_on(5, 5, 5, 5, Direction::S);
    let mut vec = Vec::new();
    vec.push(RunToward::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 5.0 on the X-axis and 5.0 on the Y-axis and facing South."
    );
}

#[test]
fn should_stop_move_outside_w() {
    let mut mars_rover = MarsRover::put_on(5, 5, 0, 5, Direction::W);
    let mut vec = Vec::new();
    vec.push(RunToward::make());
    mars_rover.execute_commands(&vec);
    assert_eq!(
        mars_rover.report(),
        "I'm 0.0 on the X-axis and 5 on the Y-axis and facing West."
    );
}
