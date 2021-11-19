use crate::day6::flyable::Flyable;

pub fn fly_things_2<T: Flyable>(item: &T) -> String {
    item.fly()
}
