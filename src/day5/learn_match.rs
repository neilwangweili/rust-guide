use crate::day5::coin::Coin;

pub fn match_coins(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => {
            // Executable code
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
