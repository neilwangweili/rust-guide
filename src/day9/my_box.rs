use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        Self(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
