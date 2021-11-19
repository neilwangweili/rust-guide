use crate::day6::bird::Bird;
use crate::day6::flyable::Flyable;

pub fn create_bird(wing: &str) -> Box<dyn Flyable> {
    Box::new(Bird::new(wing))
}
