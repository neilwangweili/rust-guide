use crate::day3::user::User;

pub fn learn_init_struct_3(username: String) -> User {
    let mut user = User {
        id: 1,
        email: String::from("webmaster@neilwang.wiki"),
        username: String::from("Neil"),
        active: true,
    };
    user.username = username;
    user
}
