use crate::dojo::fizz_buzz::fizz_buzz_line::FizzBuzzLine;

pub struct FizzBuzz {
    line: Vec<FizzBuzzLine>,
}

impl FizzBuzz {
    pub fn new(capacity: i32) -> Self {
        let mut line = Vec::new();
        for i in 1..=capacity {
            line.push(FizzBuzzLine::new(i));
        }
        Self { line }
    }

    pub fn report(&self, index: usize) -> &str {
        &self.line.get(index - 1).unwrap().value()
    }
}
