pub fn ownership_3() -> String {
    let s1 = String::from("Hello world!");
    let s2 = s1;
    // s1 is dropped. s1 is moved to s2.
    return s2;
}
