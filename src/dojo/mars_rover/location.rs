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

    pub(crate) fn report(&self) -> String {
        format!("I'm {} on the X-axis and {} on the Y-axis and facing {}.", self.x, self.y, self.direction)
    }
}

