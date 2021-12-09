use crate::dojo::grep::grep_line::GrepLine;
use std::fs;

pub struct Grep {
    lines: Vec<GrepLine>,
}

impl Grep {
    pub fn of(source: &str) -> Self {
        let source_string =
            fs::read_to_string(source).expect("Something went wrong reading the file");
        let a = source_string.split("\n").map(|o| GrepLine::of(o)).collect();
        Self { lines: a }
    }

    pub fn about(&self, str: &str) -> Vec<GrepLine> {
        let mut result = Vec::new();
        for line in self.lines() {
            if line.have(str) {
                result.push(line.clone());
            }
        }
        result
    }

    fn lines(&self) -> &Vec<GrepLine> {
        &self.lines
    }
}
