pub fn variable_readable() -> i32 {
    let _x = 5;
    // Error, x is not mutable.
    // x = 6;
    let mut y = 5;
    print!("{}", y);
    // Correctly, y is mutable.
    y = 6;
    y
}
