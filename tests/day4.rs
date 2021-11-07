use learn_rust::day4::ip_address_kind::IpAddressKind;
use learn_rust::day4::learn_use_enum::learn_use_enum;

#[test]
fn should_create_two_kinds_of_structs() {
    let (v4, v6) = learn_use_enum();
    assert_eq!(v4.address, "127.0.0.1");
    assert_eq!(v6.address, "::1");
}
