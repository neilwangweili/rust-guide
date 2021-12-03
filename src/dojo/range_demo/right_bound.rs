use crate::dojo::range_demo::bound::Bound;

pub struct RightBound {
    self_bound: Bound,
}

impl RightBound {
    pub fn init(bound_string: &str) -> Self {
        Self {
            self_bound: Bound::init(
                RightBound::get_element(bound_string),
                Bound::is_contains_bound(bound_string),
            ),
        }
    }

    fn get_element(bound_string: &str) -> f64 {
        bound_string[0..(bound_string.len() - 1)].parse().unwrap()
    }

    pub fn show(&self) -> String {
        format!("{}{}", self.self_bound.element(), self.get_contains_tag())
    }

    pub fn get_contains_tag(&self) -> String {
        String::from(match self.self_bound.contains() {
            true => "]",
            false => ")",
        })
    }
}
