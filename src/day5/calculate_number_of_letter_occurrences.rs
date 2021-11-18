use std::collections::HashMap;

pub fn calculate_number_of_letter_occurrences(text: &str) -> HashMap<char, i32> {
    let string = String::from(text)
        .replace(" ", "")
        .replace("!", "")
        .replace(".", "")
        .to_lowercase();
    let mut result = HashMap::new();
    for letter in string.chars() {
        let count = result.entry(letter).or_insert(0);
        *count += 1;
    }
    result
}
