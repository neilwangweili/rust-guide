pub fn slice_2(s: &str) -> (&str, &str, &str, &str) {
    (&s[1..5], &s[..5], &s[4..], &s[..])
}
