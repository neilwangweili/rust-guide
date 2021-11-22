pub fn slice_1(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // Declare ' ' to byte.
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
