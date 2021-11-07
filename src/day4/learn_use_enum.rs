use crate::day4::ip_address::IpAddress;
use crate::day4::ip_address_kind::IpAddressKind;

pub fn learn_use_enum() -> (IpAddress, IpAddress) {
    let home = IpAddress {
        kind: IpAddressKind::IPV4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::IPV6,
        address: String::from("::1"),
    };
    (home, loopback)
}
