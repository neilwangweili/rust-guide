use crate::dojo::mars_rover_journey::command::Command;
use crate::dojo::mars_rover_journey::coordinate::Coordinate;
use crate::dojo::mars_rover_journey::turn::Turn;

pub struct TurnLeft {}

impl TurnLeft {
    pub fn make() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for TurnLeft {
    fn execute(&self, coordinate: &mut Coordinate) {
        coordinate.turn_left();
    }
}

impl Turn for TurnLeft {}
