use crate::dojo::mars_rover_journey::command::Command;
use crate::dojo::mars_rover_journey::location::Location;
use crate::dojo::mars_rover_journey::run::Run;

pub struct RunBack {}

impl RunBack {
    pub fn make() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for RunBack {
    fn execute(&self, location: &mut Location) {
        location.move_back();
    }
}

impl Run for RunBack {}
