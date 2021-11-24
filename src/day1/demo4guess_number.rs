use rand::Rng;
use std::cmp::Ordering;

pub fn guess_number() -> bool {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut guess_number: i32 = 0;
    loop {
        guess_number += 1;
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => {
                println!("Too large!");
                break;
            }
            Ordering::Equal => {
                println!("Right!");
            }
        }
    }
    true
}
