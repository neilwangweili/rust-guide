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
        self.and();
    }

    pub fn and(&mut self) {
        let bounds = self.mut_bounds();
        for i in 0..bounds.len() - 1 {
            let _a: &Interval = bounds.get(i).unwrap();
        }
    }

    pub fn overlaps_range(&self, o: &Range) -> bool {
        let mut result = false;
        for bound in o.bounds() {
            for self_bound in &self.bounds {
                result |= bound.overlaps_range(self_bound);
            }
        }
        result
    }

    fn init_bounds(range_string: &str) -> Vec<Interval> {
        let trim_range_string = range_string.replace(" ", "");
        vec![Interval::init(trim_range_string)]
    }
}
