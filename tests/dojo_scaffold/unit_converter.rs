use learn_rust::dojo_scaffold::unit_converter::unit::Unit;

#[test]
fn should_1000_mm_return_1_m() {
    assert_eq!(Unit::new("mm", 1000.0).to("m"), 1.0);
}

#[test]
fn should_10_mm_return_1_cm() {
    assert_eq!(Unit::new("mm", 10.0).to("cm"), 1.0);
}

#[test]
fn should_1_mm_return_1_mm() {
    assert_eq!(Unit::new("mm", 1.0).to("mm"), 1.0);
}

#[test]
fn should_1_cm_return_10_mm() {
    assert_eq!(Unit::new("cm", 1.0).to("mm"), 10.0);
}

#[test]
fn should_1_m_return_1000_mm() {
    assert_eq!(Unit::new("m", 1.0).to("mm"), 1000.0);
}
