pub fn for_all_element<T>(vec: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    for i in vec {
        result.push(i);
    }
    result
}
