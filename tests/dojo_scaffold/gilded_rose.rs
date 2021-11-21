use learn_rust::dojo_scaffold::gilded_rose::gilded_rose::GildedRose;
use learn_rust::dojo_scaffold::gilded_rose::item::Item;
use learn_rust::dojo_scaffold::gilded_rose::pass_one_day::PassOneDay;

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
fn should_sulfras_item_forever() {
    let mut item = Item::create_item(String::from("Sulfuras"), 80, 0);
    let mut gilded_rose = GildedRose::new(vec![item]);
    gilded_rose.pass_one_day();
    assert_eq!(gilded_rose.get(0).quality(), 80);
}
