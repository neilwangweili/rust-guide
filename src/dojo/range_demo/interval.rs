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
        let (o1, o2) = Interval::swap_asc(&self, o);
        if o1.right.contains() && o2.left.contains() {
            o1.right.element() >= o2.left.element()
        } else {
            o1.right.element() > o2.left.element()
        }
    }

    pub fn show(&self) -> String {
        format!("{}, {}", self.left.show(), self.right.show())
    }

    pub fn swap_asc<'a>(o1: &'a Interval, o2: &'a Interval) -> (&'a Interval, &'a Interval) {
        match o1.left.element() < o2.left.element() {
            true => (o1, o2),
            false => (o2, o1),
        }
    }
}
