use crate::dojo_scaffold::fizz_buzz::fizz_buzz_line::FizzBuzzLine;

pub struct FizzBuzz {
    line: Vec<FizzBuzzLine>,
}

impl FizzBuzz {
    pub fn new() -> Self {
        let mut line = Vec::new();
        for i in 1..101 {
            line.push(FizzBuzzLine::new(i));
        }
        Self {
            line
        }
    }

    pub fn report(&self, index: usize) -> &str {
        &self.line.get(index - 1).unwrap().value()
    }
}
