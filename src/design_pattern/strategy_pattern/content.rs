use crate::design_pattern::strategy_pattern::less_than_zero_strategy::LessThanZeroStrategy;
use crate::design_pattern::strategy_pattern::more_than_zero_strategy::MoreThanZeroStrategy;
use crate::design_pattern::strategy_pattern::strategy::Strategy;

pub struct Content {
    strategy: Box<dyn Strategy>,
}

impl Content {
    pub fn of(input: i32) -> Self {
        Self {
            strategy: if input > 0 {
                Box::new(MoreThanZeroStrategy::new(input))
            } else {
                Box::new(LessThanZeroStrategy::new(input))
            },
        }
    }

    pub fn execute(&self) -> i32 {
        self.strategy.execute()
    }
}
