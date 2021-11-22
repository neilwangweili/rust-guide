use crate::dojo::mars_rover::command::Command;
use crate::dojo::mars_rover::location::Location;
use crate::dojo::mars_rover::turn::Turn;

pub struct TurnRight {}

impl TurnRight {
    pub fn make() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for TurnRight {
    fn execute(&self, location: &mut Location) {
        location.turn_right();
    }
}

impl Turn for TurnRight {}
