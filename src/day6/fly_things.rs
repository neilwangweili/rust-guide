use crate::day6::flyable::Flyable;

// dyn impl ? dyn means fat pointer. impl means item need to impl Flyable.
pub fn fly_things(item: &dyn Flyable) -> String {
    // pub fn fly_things(item: &impl Flyable) -> String {
    item.fly()
}
