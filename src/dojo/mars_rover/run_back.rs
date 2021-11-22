use crate::dojo::mars_rover::command::Command;
use crate::dojo::mars_rover::location::Location;
use crate::dojo::mars_rover::run::Run;

pub struct RunBack {}

impl RunBack {
    pub fn new() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for RunBack {
    fn execute(&self, location: &mut Location) {
        location.move_back();
    }
}

impl Run for RunBack {}
