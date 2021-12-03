pub struct Bound {
    element: f64,
    contains: bool,
}

impl Bound {
    pub fn init(element: f64, contains: bool) -> Self {
        Self {
            element,
            contains,
        }
    }

    pub fn is_contains_bound(contains: &str) -> bool {
        contains.contains("[") || contains.contains("]")
    }

    pub fn element(&self) -> f64 {
        self.element
    }
    pub fn contains(&self) -> bool {
        self.contains
    }

}
