pub fn ownership_1() -> &'static str {
    {
        let s = "hello world!";
    }
    // s is unusable.
    return "";
}
