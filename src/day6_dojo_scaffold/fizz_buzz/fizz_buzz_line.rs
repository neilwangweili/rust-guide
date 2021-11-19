pub struct FizzBuzzLine {
    value: String,
}

impl FizzBuzzLine {
    pub fn new(i: i32) -> Self {
        Self {
            value: i.to_string()
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
