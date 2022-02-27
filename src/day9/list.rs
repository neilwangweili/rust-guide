use crate::day9::list::List::{Cons, Nil};

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn get(&self) -> i32 {
        match self {
            Cons(value, _list) => *value,
            Nil => 0,
        }
    }
}
