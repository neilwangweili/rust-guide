use std::any::Any;
use std::ops::Add;
use crate::dojo::range_demo::interval::Interval;

pub struct Range {
    bounds: Vec<Interval>,
}

impl Range {
    pub fn init(range_string: &str) -> Self {
        Self {
            bounds: Range::init_bounds(range_string),
        }
    }

    pub fn equals(&self, that: &Range) -> bool {
        let mut result = true;
        if self.bounds().len() != that.bounds().len() {
            return false;
        }
        for i in 0..self.bounds().len() {
            result &= self.bounds().get(i).unwrap().equals(that.bounds().get(i).unwrap());
        }
        !result
    }

    pub fn show(&self) -> String {
        let mut result = String::new();
        for bound in self.bounds.iter() {
            if result == "" {
                result += &bound.show()
            } else {
                result = result + " ∪ " + &bound.show()
            }
        }
        result
    }

    pub fn mut_bounds(&mut self) -> &mut Vec<Interval> {
        &mut self.bounds
    }

    pub fn bounds(&self) -> &Vec<Interval> {
        &self.bounds
    }

    pub fn and_default(&mut self, new_range: &str) {
        let mut range = Range::init(new_range);
        self.bounds.append(range.mut_bounds());
        self.sort();
        self.and();
    }

    pub fn and_range(&mut self, new_range: &mut Range) {
        self.bounds.append(new_range.mut_bounds());
        self.sort();
        self.and();
    }

    pub fn and_interval(&mut self, new_interval: Interval) {
        self.bounds.insert(0, new_interval);
        self.sort();
        self.and();
    }

    pub fn and(&mut self) {
        if !self.over_close_range() {
            return;
        }
        let bounds = self.sort();
        for i in (0..bounds.len() - 1).rev() {
            let o1: &Interval = bounds.get(i).unwrap();
            let o2: &Interval = bounds.get(i + 1).unwrap();
            if !o1.over_close_range(o2) {
                continue;
            }
            let o1_left_element = o1.left().element();
            let o1_left_contains = o1.left().contains();
            let o1_right_element = o1.right().element();
            let o1_right_contains = o1.right().contains();
            let o2_left_element = o2.left().element();
            let o2_left_contains = o2.left().contains();
            let o2_right_element = o2.right().element();
            let o2_right_contains = o2.right().contains();
            bounds.remove(i + 1);
            bounds.remove(i);
            let left_contains = if o1_left_element == o2_left_element
                && (o1_left_contains || o2_left_contains)
            {
                "["
            } else if o1_left_element == o2_left_element && !(o1_left_contains || o2_left_contains)
            {
                "("
            } else if o1_left_element != o2_left_element && o1_left_contains {
                "["
            } else {
                "("
            };
            let right_contains = if o1_right_element == o2_right_element
                && (o1_right_contains || o2_right_contains)
            {
                "]"
            } else if o1_right_element == o2_right_element
                && !(o1_right_contains || o2_right_contains)
            {
                ")"
            } else if o1_right_element > o2_right_element && o1_right_contains {
                "]"
            } else if o1_right_element > o2_right_element && !o1_right_contains {
                ")"
            } else if o1_right_element < o2_right_element && o2_right_contains {
                "]"
            } else {
                ")"
            };
            bounds.insert(
                i,
                Interval::init(format!(
                    "{}{},{}{}",
                    left_contains,
                    o1_left_element,
                    match o1_right_element > o2_right_element {
                        true => o1_right_element,
                        false => o2_right_element,
                    },
                    right_contains
                )),
            );
        }
        self.and();
    }

    fn sort(&mut self) -> &mut Vec<Interval> {
        let bounds = self.mut_bounds();
        bounds.sort_by(|a, b| {
            PartialOrd::partial_cmp(&a.left().element(), &b.left().element()).unwrap()
        });
        bounds
    }

    pub fn overlaps_range_to_others(&self, o: &Range) -> bool {
        let mut result = false;
        for bound in o.bounds() {
            for self_bound in &self.bounds {
                result |= bound.overlaps_range(self_bound);
            }
        }
        result
    }

    fn over_close_range(&self) -> bool {
        let mut result = false;
        for i in 0..self.bounds.len() {
            let bound = &self.bounds[i];
            for j in i + 1..self.bounds.len() {
                let self_bound = &self.bounds[j];
                result |= bound.over_close_range(self_bound);
            }
        }
        result
    }

    fn init_bounds(range_string: &str) -> Vec<Interval> {
        let trim_range_string = range_string.replace(" ", "");
        vec![Interval::init(trim_range_string)]
    }
}
