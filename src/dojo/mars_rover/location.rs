use crate::dojo::mars_rover::direction::Direction;

pub struct Location {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Location {
    pub fn set(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
        }
    }

    pub fn report(&self) -> String {
        format!("I'm {} on the X-axis and {} on the Y-axis and facing {}.", self.x, self.y, self.direction)
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::S => { self.direction = Direction::W }
            Direction::E => { self.direction = Direction::S }
            Direction::N => { self.direction = Direction::E }
            Direction::W => { self.direction = Direction::N }
        };
    }
}
