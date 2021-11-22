use crate::dojo::mars_rover::command::Command;
use crate::dojo::mars_rover::location::Location;
use crate::dojo::mars_rover::turn::Turn;

pub struct TurnLeft {}

impl TurnLeft {
    pub fn new() -> TurnLeft {
        Self {}
    }
}

impl Command for TurnLeft {
    fn execute(&self, location: &mut Location) {
        location.turn_left();
    }
}

impl Turn for TurnLeft {}
