pub fn clone() -> String {
    let s1 = String::from("Hello world!");
    let _s2 = s1.clone();
    // s1 is still usable because clone create new "Hello world!" in heap
    s1
}
