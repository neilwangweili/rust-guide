use crate::dojo::range_demo::left_bound::LeftBound;
use crate::dojo::range_demo::range::Range;
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

    pub fn over_close_range(&self, o: &Interval) -> bool {
        let (o1, o2) = Interval::swap_asc(&self, o);
        if (o1.right.contains() && o2.left.contains()) || (o1.right.contains() ^ o2.left.contains())
        {
            o1.right.element() >= o2.left.element()
        } else {
            o1.right.element() > o2.left.element()
        }
    }

    pub fn show(&self) -> String {
        format!("{}, {}", self.left.show(), self.right.show())
    }

    fn swap_asc<'a>(o1: &'a Interval, o2: &'a Interval) -> (&'a Interval, &'a Interval) {
        match o1.left.element() < o2.left.element() {
            true => (o1, o2),
            false => (o2, o1),
        }
    }

    pub fn equals(&self, that: &Interval) -> bool {
        self.right().equals(that.right()) && self.left().equals(that.left())
    }

    pub fn left(&self) -> &LeftBound {
        &self.left
    }

    pub fn right(&self) -> &RightBound {
        &self.right
    }

    pub fn is_collection(&self) -> bool {
        &self.left().element() == &self.right().element()
    }

    pub fn range_contains(&self, that: &Interval) -> bool {
        if self.out_of_range(that) {
            false
        } else if self.in_range(that) {
            true
        } else if self.left_in_range_right_equals(that) {
            self.right_bound_contains(that)
        } else if self.right_in_range_left_equals(that) {
            self.left_bound_contains(that)
        } else {
            self.right_bound_contains(that) && self.right_bound_contains(that)
        }
    }

    fn left_bound_contains(&self, that: &Interval) -> bool {
        self.left().contains() || !that.left().contains()
    }

    fn out_of_range(&self, that: &Interval) -> bool {
        !self.in_range(that) && !self.right_element_equals(that) && !self.left_element_equals(that)
    }

    fn in_range(&self, that: &Interval) -> bool {
        self.left_in_range(that) && self.right_in_range(that)
    }

    fn right_in_range(&self, that: &Interval) -> bool {
        self.right().element() > that.right().element()
    }

    fn left_in_range(&self, that: &Interval) -> bool {
        self.left().element() < that.left().element()
    }

    fn left_in_range_right_equals(&self, that: &Interval) -> bool {
        self.left_in_range(that) && self.right_element_equals(that)
    }

    fn right_element_equals(&self, that: &Interval) -> bool {
        self.right().element() == that.right().element()
    }

    fn right_in_range_left_equals(&self, that: &Interval) -> bool {
        self.left_element_equals(that) && self.right_in_range(that)
    }

    fn left_element_equals(&self, that: &Interval) -> bool {
        self.left().element() == that.left().element()
    }

    fn right_bound_contains(&self, that: &Interval) -> bool {
        self.right().contains() || !that.right().contains()
    }
}
