use crate::dojo::range_demo::bound::Bound;

pub struct RightBound {
    self_bound: Bound,
}

impl RightBound {
    pub fn init(bound_string: &str) -> Self {
        Self {
            self_bound: Bound::init(
                Self::get_element(bound_string),
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
        Self::get_tag(self.self_bound.contains())
    }

    pub fn get_contains_tag_from_out(contains: bool) -> String {
        Self::get_tag(contains)
    }

    fn get_tag(contains: bool) -> String {
        String::from(match contains {
            true => "]",
            false => ")",
        })
    }

    pub fn equals(&self, that: &Self) -> bool {
        self.element() == that.element() && self.contains() == that.contains()
    }

    pub fn element(&self) -> f64 {
        self.self_bound.element()
    }

    pub fn contains(&self) -> bool {
        self.self_bound.contains()
    }
}
