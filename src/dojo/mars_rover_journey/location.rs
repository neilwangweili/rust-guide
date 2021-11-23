pub struct Location {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Location {
    pub fn set(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn report(&self) -> String {
        format!("I'm {} on the X-axis and {} on the Y-axis", self.x, self.y)
    }


    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }


    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}
