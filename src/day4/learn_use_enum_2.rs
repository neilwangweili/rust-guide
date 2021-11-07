use crate::day4::ip_address_kind_v2::IpAddressKindV2;

pub fn learn_use_enum_2() -> (IpAddressKindV2, IpAddressKindV2) {
    (IpAddressKindV2::IPV4(127, 0, 0, 1),
     IpAddressKindV2::IPV6(String::from("::1")))
}
