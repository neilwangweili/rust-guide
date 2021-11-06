pub fn references_2() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    // It is error. s2 can't edit it's value.
    // s2.push_str(" world!");
}
