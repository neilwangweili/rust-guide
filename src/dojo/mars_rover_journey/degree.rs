use crate::dojo::mars_rover_journey::direction::Direction;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::fmt::{Display, Formatter};

pub struct Degree {
    degree: f64,
}

impl Degree {

    pub fn calculate_length(&self) -> (f64, f64) {
        (self.degree.cos(), self.degree.sin())
    }

    pub fn from_direction(direction: Direction) -> Self {
        Self {
            degree: match direction {
                Direction::S => FRAC_PI_2,
                Direction::E => 0.0,
                Direction::N => (FRAC_PI_2 * 3.0),
                Direction::W => (FRAC_PI_2 * 2.0),
            },
        }
    }

    pub fn to_direction(&self) -> Direction {
        if self.degree == 0.0 {
            Direction::E
        } else if self.degree == FRAC_PI_2 {
            Direction::S
        } else if self.degree == FRAC_PI_2 * 2.0 {
            Direction::W
        } else {
            Direction::N
        }
    }

    pub fn turn_left(&mut self) {
        self.degree += FRAC_PI_2;
        self.remove_a_tau();
    }

    pub fn turn_right(&mut self) {
        self.degree += FRAC_PI_2 * 3.0;
        self.remove_a_tau();
    }

    fn remove_a_tau(&mut self) {
        if self.degree >= TAU {
            self.degree -= TAU;
        }
    }

    pub fn report(&self) -> String {
        self.to_direction().report()
    }
}
