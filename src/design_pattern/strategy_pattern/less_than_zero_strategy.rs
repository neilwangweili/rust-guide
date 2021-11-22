use crate::design_pattern::strategy_pattern::strategy::Strategy;

pub struct LessThanZeroStrategy {
    input: i32,
}

impl LessThanZeroStrategy {
    pub fn new(input: i32) -> Self {
        Self { input }
    }
}

impl Strategy for LessThanZeroStrategy {
    fn execute(&self) -> i32 {
        0 - self.input
    }
}
