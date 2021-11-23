use crate::dojo::mars_rover_journey::direction::Direction;
use crate::dojo::mars_rover_journey::location::Location;

pub struct Coordinate {
    location: Location,
    direction: Direction,
}

impl Coordinate {
    pub fn set(x: i32, y: i32, direction: Direction) -> Coordinate {
        Self {
            location: Location::set(x, y),
            direction,
        }
    }

    pub fn report(&self) -> String {
        format!(
            "{} and {}.",
            self.location.report(),
            self.direction.report()
        )
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::S => self.direction = Direction::W,
            Direction::E => self.direction = Direction::S,
            Direction::N => self.direction = Direction::E,
            Direction::W => self.direction = Direction::N,
        };
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::S => self.direction = Direction::E,
            Direction::E => self.direction = Direction::N,
            Direction::N => self.direction = Direction::W,
            Direction::W => self.direction = Direction::S,
        };
    }

    pub fn move_toward(&mut self) {
        match self.direction {
            Direction::S => self.location.set_y(self.location.y() + 1),
            Direction::E => self.location.set_x(self.location.x() + 1),
            Direction::N => self.location.set_y(self.location.y() - 1),
            Direction::W => self.location.set_x(self.location.x() - 1),
        };
    }

    pub fn move_back(&mut self) {
        match self.direction {
            Direction::S => self.location.set_y(self.location.y() - 1),
            Direction::E => self.location.set_x(self.location.x() - 1),
            Direction::N => self.location.set_y(self.location.y() + 1),
            Direction::W => self.location.set_x(self.location.x() + 1),
        };
    }
}
