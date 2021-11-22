use crate::dojo::mars_rover::area::Area;
use crate::dojo::mars_rover::command::Command;
use crate::dojo::mars_rover::direction::Direction;
use crate::dojo::mars_rover::location::Location;

pub struct MarsRover {
    area: Area,
    location: Location,
}

impl MarsRover {
    pub fn put_on(x_max: i32, y_max: i32, x: i32, y: i32, direction: Direction) -> Self {
        Self {
            area: Area::tectonic(x_max, y_max),
            location: Location::set(x, y, direction),
        }
    }

    pub fn report(&self) -> String {
        self.location.report()
    }

    pub fn execute_commands(&mut self, commands: &Vec<Box<dyn Command>>) {
        for command in commands.iter() {
            command.execute(&mut self.location);
        }
    }
}
