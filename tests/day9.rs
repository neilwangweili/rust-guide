use rust_guide::day9::list::List::{Cons, Nil};

#[test]
fn should_create_list_as_1_2_3_4() {
    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );
    assert_eq!(list.get(), 1);
}

#[test]
fn should_null_return_0() {
    let list = Nil;
    assert_eq!(list.get(), 0);
}
