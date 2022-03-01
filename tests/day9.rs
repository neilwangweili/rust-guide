use std::rc::Rc;
use rust_guide::day9::list::List::{Cons, Nil};
use rust_guide::day9::my_box::MyBox;
use rust_guide::day9::my_rc_list::MyRcList;

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

#[test]
fn should_my_box_test() {
    let x = MyBox::new(5);
    assert_eq!(*x, 5);
}

#[test]
fn my_rc_list_multi_reference_success() {
    let a = Rc::new(MyRcList::Cons(5, Rc::new(MyRcList::Cons(4, Rc::new(MyRcList::Nil)))));
    let b = MyRcList::Cons(3, Rc::clone(&a));
    let c = MyRcList::Cons(4, Rc::clone(&a));
    assert_eq!(Rc::strong_count(&a), 3);
}
