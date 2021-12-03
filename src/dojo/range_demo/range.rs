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
        self.sort();
        self.and();
    }

    pub fn and(&mut self) {
        if !self.overlaps_range() {
            return;
        }
        let bounds = self.sort();
        for i in bounds.len() - 2..=0 {
            let this: &Interval = bounds.get(i).unwrap();
            let next: &Interval = bounds.get(i + 1).unwrap();
            if !this.overlaps_range(next) {
                continue;
            }
            // let (o1, o2) = Interval::swap_asc(this, next);
        }
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

    fn overlaps_range(&self) -> bool {
        let mut result = false;
        for i in 0..self.bounds.len() {
            let bound = &self.bounds[i];
            for j in i + 1..self.bounds.len() {
                let self_bound = &self.bounds[j];
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
