use crate::day6::bird::Bird;
use crate::day6::flyable::Flyable;

pub fn bird_can_fly(wing: &str) -> String {
    Bird::new(wing).fly()
}
