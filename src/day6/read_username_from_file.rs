use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    // ? is auto parse as match.
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
