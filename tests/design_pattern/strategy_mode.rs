use learn_rust::design_pattern::strategy_mode::content::Content;

#[test]
fn absolute_of_positive_number() {
    assert_eq!(Content::of(1).execute(), 1);
}

#[test]
fn absolute_of_negative_number() {
    assert_eq!(Content::of(-2).execute(), 2);
}
