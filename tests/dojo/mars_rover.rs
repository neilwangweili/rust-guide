use rust_guide::dojo::mars_rover::command::Command;
use rust_guide::dojo::mars_rover::direction::Direction;
use rust_guide::dojo::mars_rover::mars_rover::MarsRover;
use rust_guide::dojo::mars_rover::run_back::RunBack;
use rust_guide::dojo::mars_rover::run_toward::RunToward;
use rust_guide::dojo::mars_rover::turn_left::TurnLeft;
use rust_guide::dojo::mars_rover::turn_right::TurnRight;

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

#[test]
fn should_mars_rover_turn_left_to_south_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(TurnLeft::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_turn_left_to_west_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(TurnLeft::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_turn_left_to_north_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(TurnLeft::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_turn_right_to_west_at_north() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::N);
    let mut vec = Vec::new();
    vec.push(TurnRight::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_turn_right_to_south_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(TurnRight::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_turn_right_to_east_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(TurnRight::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing East.");
}

#[test]
fn should_mars_rover_turn_right_to_north_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(TurnRight::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 3 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_move_toward_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(RunToward::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 4 on the X-axis and 3 on the Y-axis and facing East.");
}

#[test]
fn should_mars_rover_move_toward_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(RunToward::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 4 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_move_toward_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(RunToward::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 2 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_move_back_at_east() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::E);
    let mut vec = Vec::new();
    vec.push(RunBack::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 2 on the X-axis and 3 on the Y-axis and facing East.");
}

#[test]
fn should_mars_rover_move_back_at_south() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::S);
    let mut vec = Vec::new();
    vec.push(RunBack::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 2 on the Y-axis and facing South.");
}

#[test]
fn should_mars_rover_move_back_at_west() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::W);
    let mut vec = Vec::new();
    vec.push(RunBack::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 4 on the X-axis and 3 on the Y-axis and facing West.");
}

#[test]
fn should_mars_rover_move_back_at_north() {
    let mut mars_rover = MarsRover::put_on(10, 10, 3, 3, Direction::N);
    let mut vec = Vec::new();
    vec.push(RunBack::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 3 on the X-axis and 4 on the Y-axis and facing North.");
}

#[test]
fn should_mars_rover_run_successfully() {
    let mut mars_rover = MarsRover::put_on(10, 15, 3, 3, Direction::N);
    let mut vec: Vec<Box<dyn Command>> = Vec::new();
    vec.push(RunToward::new());
    vec.push(RunToward::new());
    vec.push(TurnLeft::new());
    vec.push(RunToward::new());
    vec.push(TurnRight::new());
    vec.push(RunBack::new());
    vec.push(TurnLeft::new());
    vec.push(TurnLeft::new());
    vec.push(RunBack::new());
    mars_rover.execute_commands(&vec);
    assert_eq!(mars_rover.report(), "I'm 4 on the X-axis and 1 on the Y-axis and facing South.");
}
