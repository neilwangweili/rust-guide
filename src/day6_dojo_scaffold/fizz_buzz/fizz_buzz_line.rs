pub struct FizzBuzzLine {
    value: String,
}

impl FizzBuzzLine {
    pub fn new(i: i32) -> Self {
        Self {
            value: Self::calculate_value(i)
        }
    }

    fn calculate_value(i: i32) -> String {
        if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
