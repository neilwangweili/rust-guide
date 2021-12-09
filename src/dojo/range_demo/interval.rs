use crate::dojo::range_demo::left_bound::LeftBound;
use crate::dojo::range_demo::right_bound::RightBound;

pub struct Interval {
    left: LeftBound,
    right: RightBound,
}

impl Interval {
    pub fn init(interval_string: String) -> Self {
        let string = interval_string.replace(" ", "");
        let elements: Vec<&str> = string.split(",").collect();
        Self {
            left: LeftBound::init(elements[0]),
            right: RightBound::init(elements[1]),
        }
    }

    pub fn get_overlaps(&self, that: &Self) -> Self {
        let (o1, o2) = Self::swap_asc(self, that);
        let (left_element, left_contains) = Self::cal_left_bound(o1, o2);
        let (right_element, right_contains) = Self::cal_right_bound(o1, o2);
        Self::init(format!(
            "{}{},{}{}",
            LeftBound::get_contains_tag_from_out(left_contains),
            left_element,
            right_element,
            RightBound::get_contains_tag_from_out(right_contains)
        ))
    }

    pub fn overlaps_range(&self, o: &Self) -> bool {
        let (o1, o2) = Self::swap_asc(&self, o);
        if Self::cover_end_points(o1, o2) {
            Self::element_eq_lt(o1, o2)
        } else {
            Self::element_lt(o1, o2)
        }
    }

    pub fn over_close_range(&self, o: &Self) -> bool {
        let (o1, o2) = Self::swap_asc(&self, o);
        if Self::cover_end_points(o1, o2) || Self::has_one_big_contains(o1, o2) {
            Self::element_eq_lt(o1, o2)
        } else {
            Self::element_lt(o1, o2)
        }
    }

    pub fn show(&self) -> String {
        format!("{}, {}", self.left.show(), self.right.show())
    }

    pub fn equals(&self, that: &Self) -> bool {
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

    pub fn range_contains(&self, that: &Self) -> bool {
        if self.out_of_range(that) {
            false
        } else if self.in_range(that) {
            true
        } else if self.left_in_range_right_equals(that) {
            self.right_bound_contains(that)
        } else if self.right_in_range_left_equals(that) {
            self.left_bound_contains(that)
        } else {
            self.left_bound_contains(that) && self.right_bound_contains(that)
        }
    }

    pub fn has_unlimited(&self) -> bool {
        self.left().has_unlimited() || self.right().has_unlimited()
    }

    fn has_one_big_contains(o1: &Self, o2: &Self) -> bool {
        o1.right.contains() ^ o2.left.contains()
    }

    fn cal_right_bound(o1: &Self, o2: &Self) -> (f64, bool) {
        (
            if Self::right_element_lt(o1, o2) {
                o2.right().element()
            } else if Self::right_element_gt(o1, o2) {
                o1.right().element()
            } else {
                o1.right().element()
            },
            if Self::right_element_lt(o1, o2) {
                o2.right().contains()
            } else if Self::right_element_gt(o1, o2) {
                o1.right().contains()
            } else {
                o1.right().contains() && o2.right().contains()
            },
        )
    }

    fn cal_left_bound(o1: &Self, o2: &Self) -> (f64, bool) {
        (
            o2.left().element(),
            if Self::left_element_lt(o1, o2) {
                o2.left().contains()
            } else {
                o1.left().contains() && o2.left().contains()
            },
        )
    }

    fn left_element_lt(o1: &Self, o2: &Self) -> bool {
        o1.left().element() < o2.left().element()
    }

    fn right_element_lt(o1: &Self, o2: &Self) -> bool {
        o2.right().element() < o1.right().element()
    }

    fn right_element_gt(o1: &Self, o2: &Self) -> bool {
        o2.right().element() > o1.right().element()
    }

    fn element_lt(o1: &Self, o2: &Self) -> bool {
        o1.right.element() > o2.left.element()
    }

    fn element_eq_lt(o1: &Self, o2: &Self) -> bool {
        o1.right.element() >= o2.left.element()
    }

    fn cover_end_points(o1: &Self, o2: &Self) -> bool {
        o1.right.contains() && o2.left.contains()
    }

    fn swap_asc<'a>(o1: &'a Self, o2: &'a Self) -> (&'a Self, &'a Self) {
        match o1.left.element() < o2.left.element() {
            true => (o1, o2),
            false => (o2, o1),
        }
    }

    fn left_bound_contains(&self, that: &Self) -> bool {
        self.left().contains() || !that.left().contains()
    }

    fn out_of_range(&self, that: &Self) -> bool {
        !self.in_range(that) && !self.right_element_equals(that) && !self.left_element_equals(that)
    }

    fn in_range(&self, that: &Self) -> bool {
        self.left_in_range(that) && self.right_in_range(that)
    }

    fn right_in_range(&self, that: &Self) -> bool {
        self.right().right_in_range(that.right())
    }

    fn left_in_range(&self, that: &Self) -> bool {
        self.left().element() < that.left().element()
    }

    fn left_in_range_right_equals(&self, that: &Self) -> bool {
        self.left_in_range(that) && self.right_element_equals(that)
    }

    fn right_element_equals(&self, that: &Self) -> bool {
        self.right().element() == that.right().element()
    }

    fn right_in_range_left_equals(&self, that: &Self) -> bool {
        self.left_element_equals(that) && self.right_in_range(that)
    }

    fn left_element_equals(&self, that: &Self) -> bool {
        self.left().element() == that.left().element()
    }

    fn right_bound_contains(&self, that: &Self) -> bool {
        self.right().contains() || !that.right().contains()
    }
}
