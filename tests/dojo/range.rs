use rust_guide::dojo::range_demo::interval::Interval;
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
fn should_create_2_6_3() {
    let range = Range::init(" (2, 6 )");
    assert_eq!(range.show(), "(2, 6)");
}

#[test]
fn should_e2_6_and_7_8_return_e2_7() {
    let mut range = Range::init(" [2, 6 )");
    range.and_default("(7,8)");
    assert_eq!(range.show(), "[2, 6) ∪ (7, 8)");
}

#[test]
fn should_e2_6_and_7_8_return_e2_7_2() {
    let mut range = Range::init("(7,8)");
    range.and_default(" [2, 6 )");
    assert_eq!(range.show(), "[2, 6) ∪ (7, 8)");
}

#[test]
fn should_e2_6_and_7_8_return_e2_7_range() {
    let mut range = Range::init("(7,8)");
    range.and_range(&mut Range::init(" [2, 6 )"));
    assert_eq!(range.show(), "[2, 6) ∪ (7, 8)");
}

#[test]
fn should_e2_6_and_7_8_return_e2_7_interval() {
    let mut range = Range::init("(7,8)");
    range.and_interval(Interval::init(String::from("[2,6)")));
    assert_eq!(range.show(), "[2, 6) ∪ (7, 8)");
}

#[test]
fn should_e2_6_and_7_8_and_5_8e_return_e2_e8() {
    let mut range = Range::init("(7,8)");
    range.and_interval(Interval::init(String::from("[2,6)")));
    range.and_interval(Interval::init(String::from("[6,8]")));
    assert_eq!(range.show(), "[2, 8]");
}

#[test]
fn should_e2_6_and_3_8_return_e2_8() {
    let mut range = Range::init("(3,8)");
    range.and_default(" [2, 6 )");
    assert_eq!(range.show(), "[2, 8)");
}

#[test]
fn should_e2_6_and_2_5_return_e2_6() {
    let mut range = Range::init("[2,6)");
    range.and_default(" (2,5)");
    assert_eq!(range.show(), "[2, 6)");
}

#[test]
fn should_2_6_and_2_5_return_2_6() {
    let mut range = Range::init("(2,6)");
    range.and_default(" (2,5)");
    assert_eq!(range.show(), "(2, 6)");
}

#[test]
fn should_2_6_and_e2_6_return_e2_6() {
    let mut range = Range::init("(2,6)");
    range.and_default(" [2,6)");
    assert_eq!(range.show(), "[2, 6)");
}

#[test]
fn should_2_6_and_3_5_return_2_6() {
    let mut range = Range::init("(2,6)");
    range.and_default(" (3,5)");
    assert_eq!(range.show(), "(2, 6)");
}

#[test]
fn should_2_6_and_3_e7_return_2_e7() {
    let mut range = Range::init("(2,6)");
    range.and_default(" (3,7]");
    assert_eq!(range.show(), "(2, 7]");
}

#[test]
fn should_2_e7_and_3_e4_return_2_e7() {
    let mut range = Range::init("(2,7]");
    range.and_default(" (3,4)");
    assert_eq!(range.show(), "(2, 7]");
}

#[test]
fn should_2_7_and_3_7_return_2_7() {
    let mut range = Range::init("(2,7)");
    range.and_default(" (3,7)");
    assert_eq!(range.show(), "(2, 7)");
}

#[test]
fn should_2_e7_and_3_7_return_2_e7() {
    let mut range = Range::init("(2,7]");
    range.and_default(" (3,7)");
    assert_eq!(range.show(), "(2, 7]");
}

#[test]
fn should_2_7_and_3_e7_return_2_e7() {
    let mut range = Range::init("(2,7)");
    range.and_default(" (3,7]");
    assert_eq!(range.show(), "(2, 7]");
}

#[test]
fn should_2_e7_and_3_e7_return_2_e7() {
    let mut range = Range::init("(2,7]");
    range.and_default(" (3,7]");
    assert_eq!(range.show(), "(2, 7]");
}

#[test]
fn should_e2_5_not_overlap_e7_10() {
    let range = Range::init("[2,5)");
    let range_2 = Range::init("[7,10)");
    assert_eq!(range.overlaps_range_to_others(&range_2), false);
}

#[test]
fn should_e2_5_not_overlap_e7_10_2() {
    let range = Range::init("[7,10)");
    let range_2 = Range::init("[2,5)");
    assert_eq!(range.overlaps_range_to_others(&range_2), false);
}

#[test]
fn should_2_10_overlap_e3_5() {
    let range = Range::init("[2,10)");
    let range_2 = Range::init("[3,5)");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_e3_5_overlap_e3_5() {
    let range = Range::init("[3,5)");
    let range_2 = Range::init("[3,5)");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_e2_5_overlap_with_e3_10() {
    let range = Range::init("[2,5)");
    let range_2 = Range::init("[3,10)");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_e3_5_overlap_with_e2_10() {
    let range = Range::init("[3,5)");
    let range_2 = Range::init("[2,10)");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_e3_e5_overlap_with_e5_10() {
    let range = Range::init("[3,5]");
    let range_2 = Range::init("[5,10]");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_e3_e5_not_overlap_with_5_10() {
    let range = Range::init("[3,5]");
    let range_2 = Range::init("(5,10]");
    assert_eq!(range.overlaps_range_to_others(&range_2), false);
}

#[test]
fn e3_5_equals_e3_5() {
    let range = Range::init("[3,5]");
    let range_2 = Range::init("[3,5]");
    assert_eq!(range.equals(&range_2), true);
}

#[test]
fn e3_5_7_8_not_equals_e3_5() {
    let mut range = Range::init("[3,5]");
    range.and_default("(7,8)");
    let range_2 = Range::init("(3,5]");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn e2_10_not_equals_e3_5() {
    let range = Range::init("[2,10)");
    let range_2 = Range::init("[3,5)");
    assert_eq!(range.equals(&range_2), false);
}
