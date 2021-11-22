use crate::dojo::gilded_rose_demo::pass_one_day::PassOneDay;

pub struct BackstagePassItem {
    name: String,
    quality: i32,
    sell_in: i32,
}

impl BackstagePassItem {
    pub fn of(name: String, quality: i32, sell_in: i32) -> Box<dyn PassOneDay> {
        Box::new(Self {
            name,
            quality,
            sell_in,
        })
    }
}

impl PassOneDay for BackstagePassItem {
    fn pass_one_day(&mut self) {
        self.sell_in -= 1;
        if self.sell_in >= 10 {
            self.quality -= 1;
        } else if self.sell_in >= 5 {
            self.quality += 2;
        } else if self.sell_in >= 0 {
            self.quality += 3;
        } else {
            self.quality = 0;
        }
        if self.quality < 0 {
            self.quality = 0;
        }
        if self.quality > 50 {
            self.quality = 50;
        }
    }

    fn quality(&self) -> i32 {
        self.quality
    }
}
