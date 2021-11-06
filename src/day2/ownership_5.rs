pub fn ownership_5() -> String {
    // s1 accepted ownership from give_ownership()
    let s1 = give_ownership();
    // give s1 to takes_and_gives_back(String) and s1 is unusable by this action.
    // s2 accepted ownership from give_ownership()
    let s2 = takes_and_gives_back(s1);
    return s2;
}

fn takes_and_gives_back(s: String) -> String {
    return s;
    // Move ownership of a to a variable which accepted from a level above the method.
}

fn give_ownership() -> String {
    let a = String::from("Hello world!");
    return a;
    // Move ownership of a to a variable which accepted from a level above the method.
}
