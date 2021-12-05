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

#[test]
fn e2_5_not_equals_e3_10() {
    let range = Range::init("[2,5)");
    let range_2 = Range::init("[3,10)");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn e3_5_not_equals_e2_10() {
    let range = Range::init("[3,5)");
    let range_2 = Range::init("[2,10)");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn e3_5_not_equals_3_5() {
    let range = Range::init("[3,5)");
    let range_2 = Range::init("(3,5)");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn m3_5_not_equals_3_e5() {
    let range = Range::init("(3,5)");
    let range_2 = Range::init("(3,5]");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn e3_e5_not_equals_e3_e6() {
    let range = Range::init("[3,5]");
    let range_2 = Range::init("[3,6]");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn e3_e5_not_equals_e2_e5() {
    let range = Range::init("[3,5]");
    let range_2 = Range::init("[2,5]");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn should_create_collection() {
    let range = Range::init("{1,2,3,4,5}");
    assert_eq!(range.show(), "{1, 2, 3, 4, 5}");
}

#[test]
fn should_create_collection_with_interval() {
    let mut range = Range::init("{1,2,3,4,5}");
    range.and_default("[-1,0)");
    assert_eq!(range.show(), "[-1, 0) ∪ {1, 2, 3, 4, 5}");
}

#[test]
fn should_create_collection_add_interval_correctly() {
    let mut range = Range::init("{1,2,3,4,5}");
    range.and_default("[-1,1.5)");
    assert_eq!(range.show(), "[-1, 1.5) ∪ {2, 3, 4, 5}");
}

#[test]
fn should_e2_6_contains_2_4() {
    let range = Range::init("[2,6)");
    let range_2 = Range::init("(2,4)");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_e2_6_not_contains_n1_1_6_10() {
    let range = Range::init("[2,6)");
    let range_2 = Range::init("{-1,1,6,10}");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_2_6_contains_2_6() {
    let range = Range::init("(2,6)");
    let range_2 = Range::init("(2,6)");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_e2_6_contains_2_6() {
    let range = Range::init("[2,6)");
    let range_2 = Range::init("(2,6)");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_2_e6_contains_2_6() {
    let range = Range::init("(2,6]");
    let range_2 = Range::init("(2,6)");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_e2_e6_contains_e2_6() {
    let range = Range::init("[2,6]");
    let range_2 = Range::init("[2,6)");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_e2_e6_contains_2_e6() {
    let range = Range::init("[2,6]");
    let range_2 = Range::init("(2,6]");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_e2_e6_contains_e2_e6() {
    let range = Range::init("[2,6]");
    let range_2 = Range::init("[2,6]");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_2_6_not_contains_e2_6() {
    let range = Range::init("(2,6)");
    let range_2 = Range::init("[2,6)");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_2_6_not_contains_2_e6() {
    let range = Range::init("(2,6)");
    let range_2 = Range::init("(2,6]");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_2_6_not_contains_e2_e6() {
    let range = Range::init("(2,6)");
    let range_2 = Range::init("[2,6]");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_e2_6_not_contains_e2_e6() {
    let range = Range::init("[2,6)");
    let range_2 = Range::init("[2,6]");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_2_e6_not_contains_e2_e6() {
    let range = Range::init("(2,6]");
    let range_2 = Range::init("[2,6]");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_2_e6_contains_n_3_4_5_6() {
    let range = Range::init("(2,6]");
    let range_2 = Range::init("{3,4,5,6}");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_2_e6_not_contains_n_2() {
    let range = Range::init("(2,6]");
    let range_2 = Range::init("{2}");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_e2_6_return_2_3_4_5() {
    let range = Range::init("[2,6)");
    assert_eq!(range.get_all_points(), "{2, 3, 4, 5}");
}

#[test]
fn should_init_empty() {
    let range = Range::init("");
    assert_eq!(range.show(), "∅");
}

#[test]
fn should_empty_equals_empty() {
    let range = Range::init("");
    let range_2 = Range::init("");
    assert_eq!(range.equals(&range_2), true);
}

#[test]
fn should_empty_not_equals_others() {
    let range = Range::init("");
    let range_2 = Range::init("{1}");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn should_others_not_equals_empty() {
    let range = Range::init("{1}");
    let range_2 = Range::init("");
    assert_eq!(range.equals(&range_2), false);
}

#[test]
fn should_empty_add_others() {
    let mut range = Range::init("");
    let mut range_2 = Range::init("{1}");
    range.and_range(&mut range_2);
    assert_eq!(range.show(), "{1}");
}

#[test]
fn should_others_add_empty() {
    let mut range = Range::init("{1}");
    let mut range_2 = Range::init("");
    range.and_range(&mut range_2);
    assert_eq!(range.show(), "{1}");
}

#[test]
fn should_empty_add_empty() {
    let mut range = Range::init("");
    let mut range_2 = Range::init("");
    range.and_range(&mut range_2);
    assert_eq!(range.show(), "∅");
}

#[test]
fn should_empty_overlaps_range_to_others() {
    let range = Range::init("");
    let range_2 = Range::init("{1}");
    assert_eq!(range.overlaps_range_to_others(&range_2), false);
}

#[test]
fn should_others_overlaps_range_to_empty() {
    let range = Range::init("{1}");
    let range_2 = Range::init("");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_empty_overlaps_range_to_empty() {
    let range = Range::init("");
    let range_2 = Range::init("");
    assert_eq!(range.overlaps_range_to_others(&range_2), true);
}

#[test]
fn should_empty_range_contains_others() {
    let range = Range::init("");
    let range_2 = Range::init("{1}");
    assert_eq!(range.range_contains(&range_2), false);
}

#[test]
fn should_empty_range_contains_empty() {
    let range = Range::init("");
    let range_2 = Range::init("");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_others_range_contains_empty() {
    let range = Range::init("{1}");
    let range_2 = Range::init("");
    assert_eq!(range.range_contains(&range_2), true);
}

#[test]
fn should_empty_get_all_points() {
    let range = Range::init("");
    assert_eq!(range.get_all_points(), "∅");
}

#[test]
fn should_1_2_1_4_get_all_points() {
    let range = Range::init("(1.2,1.4)");
    assert_eq!(range.get_all_points(), "∅");
}

#[test]
fn should_1_3_or_2_4_return_2_3() {
    let mut range = Range::init("(1,3)");
    let range_2 = Range::init("(2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 3)");
}

#[test]
fn should_2_4_or_1_3_return_2_3() {
    let mut range = Range::init("(2,4)");
    let range_2 = Range::init("(1,3)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 3)");
}

#[test]
fn should_2_4_or_2_4_return_2_4() {
    let mut range = Range::init("(2,4)");
    let range_2 = Range::init("(2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_e2_4_or_2_4_return_2_4() {
    let mut range = Range::init("[2,4)");
    let range_2 = Range::init("(2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_2_4_or_e2_4_return_2_4() {
    let mut range = Range::init("(2,4)");
    let range_2 = Range::init("[2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_2_e4_or_2_4_return_2_4() {
    let mut range = Range::init("(2,4]");
    let range_2 = Range::init("(2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_2_4_or_2_e4_return_2_4() {
    let mut range = Range::init("(2,4)");
    let range_2 = Range::init("(2,4]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_e2_e4_or_2_4_return_2_4() {
    let mut range = Range::init("[2,4]");
    let range_2 = Range::init("(2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_2_4_or_e2_e4_return_2_4() {
    let mut range = Range::init("(2,4)");
    let range_2 = Range::init("[2,4]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4)");
}

#[test]
fn should_e2_4_or_e2_e4_return_e2_4() {
    let mut range = Range::init("[2,4)");
    let range_2 = Range::init("[2,4]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "[2, 4)");
}

#[test]
fn should_2_e4_or_e2_e4_return_2_e4() {
    let mut range = Range::init("(2,4]");
    let range_2 = Range::init("[2,4]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4]");
}

#[test]
fn should_e2_e4_or_e2_4_return_e2_4() {
    let mut range = Range::init("[2,4]");
    let range_2 = Range::init("[2,4)");
    range.or_range(&range_2);
    assert_eq!(range.show(), "[2, 4)");
}

#[test]
fn should_e2_e4_or_2_e4_return_2_e4() {
    let mut range = Range::init("[2,4]");
    let range_2 = Range::init("(2,4]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 4]");
}

#[test]
fn should_e2_e4_or_e2_e4_return_e2_e4() {
    let mut range = Range::init("[2,4]");
    let range_2 = Range::init("[2,4]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "[2, 4]");
}

#[test]
fn should_e2_e4_or_e4_e5_return_n4() {
    let mut range = Range::init("[2,4]");
    let range_2 = Range::init("[4,5]");
    range.or_range(&range_2);
    assert_eq!(range.show(), "{4}");
}

#[test]
fn should_e2_e4_or_4_e5_return_n4() {
    let mut range = Range::init("[2,4]");
    range.or_default("(4,5]");
    assert_eq!(range.show(), "∅");
}

#[test]
fn should_2_4_5_7_or_1_3_n6_return_2_3() {
    let mut range = Range::init("(2,4)");
    range.and_default("(5,7)");
    let mut range_2 = Range::init("(1,3)");
    range_2.and_default("{6}");
    range.or_range(&range_2);
    assert_eq!(range.show(), "(2, 3) ∪ {6}");
}

#[test]
fn integration_or() {
    let mut range = Range::init("[1,2]");
    range.and_default("[3,4]");
    range.and_default("{5,6,7,8,9,10}");
    range.and_default("[9,10)");
    let mut range_2 = Range::init("[1.5,3.5)");
    range_2.and_default("[3.7,4)");
    range_2.and_default("[5,11]");
    range.or_range(&range_2);
    assert_eq!(
        range.show(),
        "[1.5, 2] ∪ [3, 3.5) ∪ [3.7, 4) ∪ [9, 10] ∪ {5, 6, 7, 8}"
    );
}
