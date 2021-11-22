pub struct FizzBuzzLine {
    value: String,
}

impl FizzBuzzLine {
    pub fn new(i: i32) -> Self {
        Self {
            value: Self::calculate_value(i),
        }
    }

    fn calculate_value(i: i32) -> String {
        if Self::has_three(i) && Self::has_five(i) {
            String::from("FizzBuzz")
        } else if Self::has_three(i) {
            String::from("Fizz")
        } else if Self::has_five(i) {
            String::from("Buzz")
        } else {
            i.to_string()
        }
    }

    fn has_five(i: i32) -> bool {
        i % 5 == 0 || i.to_string().contains('5')
    }

    fn has_three(i: i32) -> bool {
        i % 3 == 0 || i.to_string().contains('3')
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
