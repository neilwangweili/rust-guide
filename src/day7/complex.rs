use std::ops::Add;

pub struct Complex {
    a: f64,
    b: f64,
}

impl Complex {
    pub fn a(&self) -> f64 {
        self.a
    }
    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a() + rhs.a(),
            b: self.b() + rhs.b(),
        }
    }
}
