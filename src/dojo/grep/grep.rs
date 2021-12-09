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
        self.lines()
            .iter()
            .filter(|o| o.have(str))
            .cloned()
            .collect()
    }

    fn lines(&self) -> &Vec<GrepLine> {
        &self.lines
    }
}
