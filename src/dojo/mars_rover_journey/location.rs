pub struct Location {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Location {
    pub fn set(x: i32, y: i32) -> Self {
        let x = x as f64;
        let y = y as f64;
        Self { x, y }
    }

    pub fn report(&self) -> String {
        format!("I'm {:.1} on the X-axis and {:.1} on the Y-axis", self.x, self.y)
    }


    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }


    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}
