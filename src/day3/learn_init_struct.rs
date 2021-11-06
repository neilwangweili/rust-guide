pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub active: bool,
}

pub fn learn_init_struct() -> User {
    return User {
        id: 1,
        email: String::from("webmaster@neilwang.wiki"),
        username: String::from("Neil"),
        active: true,
    };
}
