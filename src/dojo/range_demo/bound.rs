pub struct Bound {
    element: f64,
    contains: bool,
}

impl Bound {
    pub fn init(bound_string: &str) -> Self {
        Self {
            element: Bound::get_element(bound_string),
            contains: Bound::is_contains_bound(bound_string),
        }
    }

    fn is_contains_bound(contains: &str) -> bool {
        contains.contains("[") || contains.contains("]")
    }

    fn is_left(bound_string: &str) -> bool {
        bound_string.contains("[") || bound_string.contains("(")
    }

    fn get_element(bound_string: &str) -> f64 {
        if Bound::is_left(bound_string) {
            bound_string[1..].parse().unwrap()
        } else {
            bound_string[0..(bound_string.len() - 1)].parse().unwrap()
        }
    }

    pub fn element(&self) -> f64 {
        self.element
    }

    pub fn contains(&self) -> bool {
        self.contains
    }

}
