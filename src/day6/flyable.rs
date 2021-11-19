pub trait Flyable {
    fn fly(&self) -> String {
        // Default implementation.
        String::from("Can fly...")
    }
}
