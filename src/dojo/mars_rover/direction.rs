use std::fmt::{Display, Formatter};

pub enum Direction {
    // Right S E N W
    S,
    E,
    N,
    W,
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
