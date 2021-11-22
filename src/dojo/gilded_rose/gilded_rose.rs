use crate::dojo::gilded_rose::pass_one_day::PassOneDay;

pub struct GildedRose {
    items: Vec<Box<dyn PassOneDay>>,
}

impl GildedRose {
    pub fn new(items: Vec<Box<dyn PassOneDay>>) -> Self {
        Self {
            items
        }
    }


    pub fn get(&self, index: usize) -> &Box<dyn PassOneDay> {
        (&self.items).get(index).unwrap()
    }
}

impl PassOneDay for GildedRose {
    fn pass_one_day(&mut self) {
        for item in self.items.iter_mut() {
            item.pass_one_day();
        }
    }

    fn quality(&self) -> i32 {
        0
    }
}
