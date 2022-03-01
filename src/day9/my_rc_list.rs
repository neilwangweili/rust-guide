use std::rc::Rc;

pub enum MyRcList {
    Cons(i32, Rc<MyRcList>),
    Nil,
}
