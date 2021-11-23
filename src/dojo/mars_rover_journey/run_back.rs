use crate::dojo::mars_rover_journey::command::Command;
use crate::dojo::mars_rover_journey::coordinate::Coordinate;
use crate::dojo::mars_rover_journey::run::Run;

pub struct RunBack {}

impl RunBack {
    pub fn make() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for RunBack {
    fn execute(&self, coordinate: &mut Coordinate) {
        coordinate.move_back();
    }
}

impl Run for RunBack {}
