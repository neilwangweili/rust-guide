use std::fs;
use std::str::Split;

pub struct FileScanner {
    file_string: String,
}

impl FileScanner {
    pub fn of(source: &str) -> FileScanner {
        Self {
            file_string: fs::read_to_string(source).expect("Something went wrong reading the file"),
        }
    }

    pub fn scan(&self) -> Split<&'static str> {
        self.file_string().split("\n")
    }

    fn file_string(&self) -> &str {
        &self.file_string
    }
}
