pub fn references_4() -> String {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // Scope of r1, r2 is finished
    println!("{} and {}", r1, r2);
    // No problem
    let r3 = &mut s;
    r3.to_string()
}
