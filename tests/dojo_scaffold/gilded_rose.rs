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