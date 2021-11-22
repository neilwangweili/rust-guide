use crate::dojo::gilded_rose::pass_one_day::PassOneDay;

pub struct AgedBrieItem {
    name: String,
    quality: i32,
    sell_in: i32,
}

impl AgedBrieItem {
    pub fn new(name: String, quality: i32, sell_in: i32) -> Box<dyn PassOneDay> {
        Box::new(Self {
            name,
            quality,
            sell_in,
        })
    }
}

impl PassOneDay for AgedBrieItem {
    fn pass_one_day(&mut self) {
        self.sell_in -= 1;
        match self.sell_in >= 0 {
            true => self.quality += 1,
            false => self.quality += 2,
        }
        if self.quality > 50 {
            self.quality = 50;
        }
    }

    fn quality(&self) -> i32 {
        self.quality
    }
}
