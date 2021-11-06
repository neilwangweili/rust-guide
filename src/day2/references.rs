pub fn references() -> (String, String) {
    let s1 = String::from("Hello world!");
    // s1 is usable because borrow_string(s: &String) only borrow it's value
    let s2 = borrow_string(&s1);
    (s1, s2)
}

fn borrow_string(s: &String) -> String {
    return String::from(s);
}
