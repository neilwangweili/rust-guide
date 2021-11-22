use crate::design_pattern::strategy_mode::strategy::Strategy;

pub struct MoreThanZeroStrategy {
    input: i32,
}

impl MoreThanZeroStrategy {
    pub fn new(input: i32) -> Self {
        Self {
            input
        }
    }
}

impl Strategy for MoreThanZeroStrategy {
    fn execute(&self) -> i32 {
        self.input
    }
}
