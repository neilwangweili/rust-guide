use crate::dojo::mars_rover_journey::area::Area;

pub struct Location {
    x: f64,
    y: f64,
    area: Area,
}

impl Location {
    pub fn set(x: i32, y: i32, area: Area) -> Self {
        let x = x as f64;
        let y = y as f64;
        Self { x, y, area }
    }

    pub fn report(&self) -> String {
        format!(
            "I'm {:.1} on the X-axis and {:.1} on the Y-axis",
            self.x, self.y
        )
    }

    pub fn move_toward(&mut self, x: f64, y: f64) {
        let target_x = self.x + x;
        let target_y = self.y + y;
        if self.area.outside(target_x, target_y) {
            return;
        }
        self.x = target_x;
        self.y = target_y;
    }
}
