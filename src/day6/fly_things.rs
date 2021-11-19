use crate::day6::flyable::Flyable;

pub fn fly_things(item: &impl Flyable) -> String {
    item.fly()
}
