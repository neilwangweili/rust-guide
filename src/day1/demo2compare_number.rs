use std::cmp::Ordering;
// use std::io;

pub fn compare_number(guess: u32) -> bool {
    // Read number from Terminal.
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Fail to read line.");

    //Random number form 1 to 100. Not include 101.
    // let secret_number = rand::thread_rng().gen_range(1..101);
    let secret_number = 5;
    return match guess.cmp(&secret_number) {
        Ordering::Less => false,
        Ordering::Greater => false,
        Ordering::Equal => true,
    };
}
