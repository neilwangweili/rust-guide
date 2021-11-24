pub fn for_loop() -> i32 {
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("{}", element);
    }
    50
}
