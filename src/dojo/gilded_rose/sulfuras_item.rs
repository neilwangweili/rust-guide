use crate::dojo::gilded_rose::pass_one_day::PassOneDay;

pub struct SulfurasItem {
    name: String,
    quality: i32,
    sell_in: i32,
}

impl SulfurasItem {
    pub fn new(name: String, quality: i32, sell_in: i32) -> Box<dyn PassOneDay> {
        Box::new(SulfurasItem {
            name,
            quality,
            sell_in,
        })
    }
}

impl PassOneDay for SulfurasItem {
    fn pass_one_day(&mut self) {
        self.quality = 80;
    }

    fn quality(&self) -> i32 {
        self.quality
    }
}
