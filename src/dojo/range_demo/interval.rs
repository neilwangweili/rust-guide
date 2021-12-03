use crate::dojo::range_demo::left_bound::LeftBound;
use crate::dojo::range_demo::right_bound::RightBound;

pub struct Interval {
    left: LeftBound,
    right: RightBound,
}

impl Interval {
    pub fn init(interval_string: String) -> Self {
        let elements: Vec<&str> = interval_string.split(",").collect();
        Self {
            left: LeftBound::init(elements[0]),
            right: RightBound::init(elements[1]),
        }
    }

    pub fn overlaps_range(&self, o: &Interval) -> bool {
        false
    }

    pub fn show(&self) -> String {
        format!("{}, {}", self.left.show(), self.right.show())
    }
}
