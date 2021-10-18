pub fn break_with_numbers(mut x: i32) -> i32 {
    return loop {
        x += 1;
        if x == 10 {
            break x * 2;
        };
    };
}
