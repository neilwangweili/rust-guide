use crate::dojo::range_demo::bound::Bound;

pub struct Interval {
    left: Bound,
    right: Bound,
}

impl Interval {
    pub fn init(interval_string: String) -> Self {
        let elements: Vec<&str> = interval_string.split(",").collect();
        Self {
            left: Bound::init(elements[0]),
            right: Bound::init(elements[1]),
        }
    }

    pub fn show(&self) -> String {
        format!("{}, {}", self.report_left(), self.report_right())
    }

    fn report_left(&self) -> String {
        format!(
            "{}{}",
            match self.left.contains() {
                true => "[",
                false => "(",
            },
            self.left.element()
        )
    }

    fn report_right(&self) -> String {
        format!(
            "{}{}",
            self.right.element(),
            match self.right.contains() {
                true => "]",
                false => ")",
            }
        )
    }
}
