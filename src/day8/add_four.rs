pub fn add_four(x: i32) -> i32 {
    let add_one_v1 = |x: i32| -> i32 {
        x + 1
    };
    let add_one_v2 = |x: i32| {
        x + 1
    };
    let add_one_v3 = |x| {
        x + 1
    };
    let add_one_v4 = |x| x + 1;
    add_one_v1(add_one_v2(add_one_v3(add_one_v4(x))))
}
