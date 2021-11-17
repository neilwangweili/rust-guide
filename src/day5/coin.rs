use crate::day5::us_state::UsState;

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    AnotherEnumEg(UsState),
}
