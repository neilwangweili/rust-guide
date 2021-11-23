use std::fmt::{Display, Formatter};

pub enum Direction {
    S,
    E,
    N,
    W,
}

impl Direction {
    pub fn report(&self) -> String {
        format!("facing {}", self)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Direction::S => "South",
            Direction::E => "East",
            Direction::N => "North",
            Direction::W => "West",
        };
        Display::fmt(display, f)
    }
}
