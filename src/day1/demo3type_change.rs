pub fn type_change(guess: &str) -> u32 {
    let guess: u32 = guess.trim().parse().expect("Fail to convert to number.");
    guess
}
