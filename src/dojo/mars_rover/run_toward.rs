use crate::dojo::mars_rover::command::Command;
use crate::dojo::mars_rover::location::Location;
use crate::dojo::mars_rover::run::Run;

pub struct RunToward {}

impl RunToward {
    pub fn make() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for RunToward {
    fn execute(&self, location: &mut Location) {
        location.move_toward();
    }
}

impl Run for RunToward {}
