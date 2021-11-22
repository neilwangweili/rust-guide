use crate::day6::flyable::Flyable;

pub struct Bird {
    pub wing: String,
}

impl Bird {
    pub fn new(wing: &str) -> Self {
        Self {
            wing: String::from(wing),
        }
    }
}

impl Flyable for Bird {
    fn fly(&self) -> String {
        String::from("Bird can fly with ") + &self.wing + " wings."
    }
}
