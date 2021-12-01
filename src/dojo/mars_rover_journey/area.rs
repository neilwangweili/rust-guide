pub struct Area {
    x: f64,
    y: f64,
}

impl Area {
    pub fn outside(&self, x: f64, y: f64) -> bool {
        x > self.x || x < 0.0 || y > self.y || y < 0.0
    }
}

impl Area {
    pub fn tectonic(x: i32, y: i32) -> Self {
        Self { x: x as f64, y: y as f64 }
    }
}
