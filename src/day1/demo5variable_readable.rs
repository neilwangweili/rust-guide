pub fn variable_readable() -> i32 {
    let x = 5;
    // Error, x is not mutable.
    // x = 6;
    let mut y = 5;
    // Correctly, y is mutable.
    y = 6;
    return y;
}
