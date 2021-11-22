pub fn ownership_1() -> &'static str {
    {
        let _s = "hello world!";
    }
    // s is unusable.
    return "";
}
