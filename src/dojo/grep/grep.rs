use crate::dojo::grep::file_scanner::FileScanner;
use crate::dojo::grep::grep_line::GrepLine;

pub struct Grep {
    lines: Vec<GrepLine>,
}

impl Grep {
    pub fn of(source: &str) -> Self {
        Self {
            lines: FileScanner::of(source)
                .scan()
                .map(|o| GrepLine::of(o))
                .collect(),
        }
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
