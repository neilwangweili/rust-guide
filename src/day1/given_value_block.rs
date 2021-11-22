pub fn given_value_block() -> i32 {
    let y = {
        let x = 5;
        x + 1
    };
    y
}
