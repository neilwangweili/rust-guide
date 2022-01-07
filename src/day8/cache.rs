pub struct Cache<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    result: Option<i32>,
}

impl<T> Cache<T>
where
    T: Fn(i32) -> i32,
{
    pub fn new(calculation: T) -> Self {
        Self {
            calculation,
            result: None,
        }
    }

    pub fn get(&mut self, arg: i32) -> i32 {
        match self.result {
            Some(v) => v,
            None => {
                self.result = Some((self.calculation)(arg));
                self.result.unwrap()
            }
        }
    }
}
