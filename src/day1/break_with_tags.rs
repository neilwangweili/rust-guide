pub fn break_with_tags() -> i32 {
    let mut count: i32 = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 0 {
                break;
            } else if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    count
}
