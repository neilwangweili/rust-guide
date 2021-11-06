pub fn references_3() -> String {
    let mut s1 = String::from("Hello");
    // Warning: Can't give &mut s1 to more than one variable
    let s2 = &mut s1;
    s2.push_str(" world!");
    return s1;
}
