use std::collections::HashMap;

pub struct Unit {
    pub value: f32,
    pub unit: String,
    pub units: HashMap<String, f32>,
}

impl Unit {
    pub fn new(unit: &str, value: f32) -> Unit {
        let units = Self::init_units();
        Self {
            value: value * (&units.get(unit)).unwrap(),
            unit: String::from(unit),
            units,
        }
    }

    fn init_units() -> HashMap<String, f32> {
        let mut units = HashMap::new();
        units.insert(String::from("cm"), 10.0);
        units.insert(String::from("mm"), 1.0);
        units.insert(String::from("m"), 1000.0);
        units
    }

    pub fn to(&self, unit: &str) -> f32 {
        self.value / (&self.units.get(unit)).unwrap()
    }
}
