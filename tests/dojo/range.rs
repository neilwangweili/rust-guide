use rust_guide::dojo::range_demo::range::Range;

#[test]
fn should_create_e2_6() {
    let range = Range::init("[2,6)");
    assert_eq!(range.show(), "[2, 6)");
}

#[test]
fn should_create_e2_6_2() {
    let range = Range::init(" (2,6]");
    assert_eq!(range.show(), "(2, 6]");
}

#[test]
fn should_create_e2_6_3() {
    let range = Range::init(" (2, 6 )");
    assert_eq!(range.show(), "(2, 6)");
}
