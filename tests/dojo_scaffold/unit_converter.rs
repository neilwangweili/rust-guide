use learn_rust::dojo_scaffold::unit_converter::unit::Unit;

#[test]
fn should_1000_mm_return_1_m() {
    assert_eq!(Unit::new("mm", 1000.0).to("m"), 1.0);
}
