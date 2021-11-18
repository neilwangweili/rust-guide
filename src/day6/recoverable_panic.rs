use std::fs::File;

pub fn recoverable_panic() -> String {
    let b = File::open("./not_a_correct_direction");
    match b {
        Ok(file) => String::from("Ok"),
        Err(e) => String::from("Error")
    }
}
