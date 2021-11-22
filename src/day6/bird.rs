use crate::day6::flyable::Flyable;

pub struct Bird {
    pub wing: String,
}

impl Bird {
    pub fn new(wing: &str) -> Bird {
        return Bird {
            wing: String::from(wing),
        };
    }
}

impl Flyable for Bird {
    fn fly(&self) -> String {
        return String::from(String::from("Bird can fly with ") + &self.wing + " wings.");
    }
}
