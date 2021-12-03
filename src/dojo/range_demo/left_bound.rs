use crate::dojo::range_demo::bound::Bound;

pub struct LeftBound {
    self_bound: Bound,
}

impl LeftBound {
    pub fn init(bound_string: &str) -> Self {
        Self {
            self_bound: Bound::init(
                LeftBound::get_element(bound_string),
                Bound::is_contains_bound(bound_string),
            ),
        }
    }

    fn get_element(bound_string: &str) -> f64 {
        bound_string[1..].parse().unwrap()
    }

    pub fn show(&self) -> String {
        format!("{}{}", self.get_contains_tag(), self.self_bound.element())
    }

    pub fn get_contains_tag(&self) -> String {
        String::from(match self.self_bound.contains() {
            true => "[",
            false => "(",
        })
    }

    pub fn element(&self) -> f64 {
        self.self_bound.element()
    }

    pub fn contains(&self) -> bool {
        self.self_bound.contains()
    }
}
