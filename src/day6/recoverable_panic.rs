use std::fs::File;

pub fn recoverable_panic(path: &str) -> String {
    let b = File::open(path);
    match b {
        Ok(_file) => String::from("Ok"),
        Err(_e) => String::from("Error"),
    }
}
