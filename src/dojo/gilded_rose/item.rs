use crate::dojo::gilded_rose::aged_brie_item::AgedBrieItem;
use crate::dojo::gilded_rose::backstage_pass_item::BackstagePassItem;
use crate::dojo::gilded_rose::gilded_rose::GildedRose;
use crate::dojo::gilded_rose::pass_one_day::PassOneDay;
use crate::dojo::gilded_rose::sulfuras_item::SulfurasItem;

pub struct Item {
    name: String,
    quality: i32,
    sell_in: i32,
}

impl Item {
    pub fn create_item(name: String, quality: i32, sell_in: i32) -> Box<dyn PassOneDay> {
        if name.eq("Sulfuras") {
            SulfurasItem::new(name, quality, sell_in)
        } else if name.eq("Aged Brie") {
            AgedBrieItem::new(name, quality, sell_in)
        } else if name.eq("Backstage pass") {
            BackstagePassItem::new(name, quality, sell_in)
        } else {
            Self::new(name, quality, sell_in)
        }
    }

    fn new(name: String, quality: i32, sell_in: i32) -> Box<dyn PassOneDay> {
        Box::new(Self {
            name,
            quality,
            sell_in,
        })
    }
}

impl PassOneDay for Item {
    fn pass_one_day(&mut self) {
        self.sell_in -= 1;
        match self.sell_in >= 0 {
            true => self.quality -= 1,
            false => self.quality -= 2,
        }
        if self.quality < 0 {
            self.quality = 0;
        }
    }

    fn quality(&self) -> i32 {
        self.quality
    }
}
