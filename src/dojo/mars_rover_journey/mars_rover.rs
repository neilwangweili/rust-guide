use crate::dojo::mars_rover_journey::area::Area;
use crate::dojo::mars_rover_journey::command::Command;
use crate::dojo::mars_rover_journey::coordinate::Coordinate;
use crate::dojo::mars_rover_journey::direction::Direction;

pub struct MarsRover {
    coordinate: Coordinate,
}

impl MarsRover {
    pub fn put_on(x_max: i32, y_max: i32, x: i32, y: i32, direction: Direction) -> Self {
        Self {
            coordinate: Coordinate::set(x, y, direction, Area::tectonic(x_max, y_max)),
        }
    }

    pub fn report(&self) -> String {
        self.coordinate.report()
    }

    pub fn execute_commands(&mut self, commands: &Vec<Box<dyn Command>>) {
        for command in commands.iter() {
            command.execute(&mut self.coordinate);
        }
    }
}
