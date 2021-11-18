use std::collections::HashMap;

pub fn build_map_from_two_vector() -> (Vec<String>, Vec<i32>) {
    let key = vec![String::from("Blue"), String::from("Yellow")];
    let value = vec![10, 50];
    (key, value)
}
