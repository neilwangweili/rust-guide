use std::collections::HashMap;

pub fn learn_entry() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores
}
