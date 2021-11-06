use crate::day3::user::User;

pub fn learn_init_struct() -> User {
    return User {
        id: 1,
        email: String::from("webmaster@neilwang.wiki"),
        username: String::from("Neil"),
        active: true,
    };
}
