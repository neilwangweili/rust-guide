use std::collections::HashMap;

pub fn init_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    map
}
