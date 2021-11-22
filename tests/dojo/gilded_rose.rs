use rust_guide::dojo::gilded_rose::gilded_rose::GildedRose;
use rust_guide::dojo::gilded_rose::item::Item;
use rust_guide::dojo::gilded_rose::pass_one_day::PassOneDay;

#[test]
fn should_common_project_pass_one_day_quality_down_one() {
    let mut item = Item::create_item(String::from("Common item"), 10, 2);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 9);
}

#[test]
fn should_common_project_pass_one_day_quality_down_two() {
    let mut item = Item::create_item(String::from("Common item"), 10, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 8);
}

#[test]
fn should_common_project_pass_one_day_quality_down_one_not_down_zero() {
    let mut item = Item::create_item(String::from("Common item"), 0, 2);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 0);
}

#[test]
fn should_common_project_pass_one_day_quality_down_two_not_down_zero() {
    let mut item = Item::create_item(String::from("Common item"), 0, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 0);
}

#[test]
fn should_aged_brie_pass_one_day_quality_plus_one() {
    let mut item = Item::create_item(String::from("Aged Brie"), 10, 2);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 11);
}

#[test]
fn should_aged_brie_pass_one_day_quality_plus_two() {
    let mut item = Item::create_item(String::from("Aged Brie"), 10, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 12);
}

#[test]
fn should_aged_brie_pass_one_day_quality_plus_one_less_50() {
    let mut item = Item::create_item(String::from("Aged Brie"), 50, 2);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 50);
}

#[test]
fn should_aged_brie_pass_one_day_quality_plus_two_less_50() {
    let mut item = Item::create_item(String::from("Aged Brie"), 50, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 50);
}

#[test]
fn should_sulfras_item_forever() {
    let mut item = Item::create_item(String::from("Sulfuras"), 80, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 80);
}

#[test]
fn should_backstage_pass_item_down_1() {
    let mut item = Item::create_item(String::from("Backstage pass"), 40, 12);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 39);
}

#[test]
fn should_backstage_pass_item_down_1_more_zero() {
    let mut item = Item::create_item(String::from("Backstage pass"), 0, 12);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 0);
}

#[test]
fn should_backstage_pass_item_plus_2_in_ten_days() {
    let mut item = Item::create_item(String::from("Backstage pass"), 0, 10);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 2);
}

#[test]
fn should_backstage_pass_item_plus_2_in_ten_days_less_50() {
    let mut item = Item::create_item(String::from("Backstage pass"), 50, 10);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 50);
}

#[test]
fn should_backstage_pass_item_plus_3_in_five_days() {
    let mut item = Item::create_item(String::from("Backstage pass"), 0, 5);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 3);
}

#[test]
fn should_backstage_pass_item_become_0_out_of_date() {
    let mut item = Item::create_item(String::from("Backstage pass"), 50, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 0);
}

#[test]
fn fix_coverage_unuseful() {
    let gilded_rose = GildedRose::new(vec![]);
    assert_eq!(gilded_rose.quality(), 0);
}
