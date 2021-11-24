pub fn ownership_4() -> i32 {
    let s = String::from("Hello world!");
    take_ownership(s);
    // s is unusable in this place because ownership of s is taken by take_ownership()
    let x = 5;
    take_copy(x);
    // x is still usable because x is created in stack.
    x
}

fn take_copy(_x: i32) {}

fn take_ownership(_s: String) {}
