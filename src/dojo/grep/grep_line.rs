pub struct GrepLine {
    line: String,
}

impl GrepLine {
    pub fn of(line: &str) -> Self {
        Self {
            line: String::from(line),
        }
    }

    pub fn show(&self) -> &str {
        self.line()
    }

    pub fn have(&self, str: &str) -> bool {
        self.line().to_lowercase().contains(str)
    }

    fn line(&self) -> &str {
        &self.line
    }
}

impl Clone for GrepLine {
    fn clone(&self) -> GrepLine {
        Self {
            line: String::from(self.line()),
        }
    }
}
