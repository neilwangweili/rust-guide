use std::fs::File;

pub fn recoverable_panic(path: &str) -> String {
    let b = File::open(path);
    match b {
        Ok(file) => String::from("Ok"),
        Err(e) => String::from("Error")
    }
}
