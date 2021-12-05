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
            result &= self
                .bounds()
                .get(i)
                .unwrap()
                .equals(that.bounds().get(i).unwrap());
        }
        result
    }

    pub fn show(&self) -> String {
        let mut result = String::new();
        let mut interval_list = Vec::new();
        for bound in self.bounds.iter() {
            if bound.is_collection() {
                interval_list.push(bound);
                continue;
            }
            if result == "" {
                result += &bound.show()
            } else {
                result = result + " ∪ " + &bound.show()
            }
        }
        for i in 0..interval_list.len() {
            if i == 0 && result == "" {
                result += "{"
            } else if i == 0 {
                result += " ∪ {"
            }
            if i < interval_list.len() - 1 {
                result += &format!("{}, ", interval_list.get(i).unwrap().left().element());
            } else {
                result += &format!("{}}}", interval_list.get(i).unwrap().left().element());
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
        self.and_range(&mut Range::init(new_range));
    }

    pub fn and_range(&mut self, new_range: &mut Range) {
        self.bounds.append(new_range.mut_bounds());
        self.sort_and();
    }

    pub fn and_interval(&mut self, new_interval: Interval) {
        self.bounds.insert(0, new_interval);
        self.sort_and();
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
            let (o1_left_element, o1_left_contains, o1_right_element, o1_right_contains) = Self::get_bounds_values(o1);
            let (o2_left_element, o2_left_contains, o2_right_element, o2_right_contains) = Self::get_bounds_values(o2);
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

    pub fn overlaps_range_to_others(&self, o: &Range) -> bool {
        let mut result = false;
        for bound in o.bounds() {
            for self_bound in &self.bounds {
                result |= bound.overlaps_range(self_bound);
            }
        }
        result
    }

    pub fn range_contains(&self, that: &Range) -> bool {
        let mut result = true;
        for bound in self.bounds().iter() {
            for that_bound in that.bounds().iter() {
                result &= bound.range_contains(that_bound);
            }
        }
        result
    }

    pub fn get_all_points(&self) -> String {
        let left = self.bounds()[0].left().element().floor() as usize;
        let right = self.bounds()[self.bounds.len() - 1]
            .right()
            .element()
            .ceil() as usize;
        Range::init(&self.create_all_points(left, right)).show()
    }

    fn get_bounds_values(o1: &Interval) -> (f64, bool, f64, bool) {
        (o1.left().element(), o1.left().contains(), o1.right().element(), o1.right().contains())
    }

    fn sort_and(&mut self) {
        self.sort();
        self.and();
    }

    fn sort(&mut self) -> &mut Vec<Interval> {
        let bounds = self.mut_bounds();
        bounds.sort_by(|a, b| {
            PartialOrd::partial_cmp(&a.left().element(), &b.left().element()).unwrap()
        });
        bounds
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
        if trim_range_string.contains("[") || trim_range_string.contains("(") {
            vec![Interval::init(trim_range_string)]
        } else {
            Range::init_collections(range_string)
        }
    }

    fn init_collections(collection_string: &str) -> Vec<Interval> {
        let str = collection_string.replace("{", "").replace("}", "");
        let mut interval = Vec::new();
        for collection in str.split(",") {
            interval.push(Interval::init(format!("[{},{}]", collection, collection)));
        }
        interval
    }

    fn create_all_points(&self, left: usize, right: usize) -> String {
        let mut matched = Vec::new();
        for i in left..right {
            if self.range_contains(&Range::init(&format!("{{{}}}", i))) {
                matched.push(i);
            }
        }
        let mut result_str = String::from("{");
        for i in 0..matched.len() {
            if i < matched.len() - 1 {
                result_str = result_str + &matched.get(i).unwrap().to_string() + ",";
            } else {
                result_str += &matched.get(i).unwrap().to_string();
            }
        }
        result_str += "}";
        result_str
    }
}
