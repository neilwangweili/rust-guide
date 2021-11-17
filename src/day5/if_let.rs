use crate::day5::coin::Coin;

pub fn is_quarter(coin: Coin) -> bool {
    if let Coin::Quarter = coin {
        true
    } else {
        false
    }
}
