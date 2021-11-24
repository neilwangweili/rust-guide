pub fn ownership_3() -> String {
    // s1 is dropped. s1 is moved to s2.
    String::from("Hello world!")
}
