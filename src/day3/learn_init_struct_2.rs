use crate::day3::user::User;

pub fn learn_init_struct_2(id: i32, username: String, email: String, b: bool) -> User {
    User {
        id,
        username,
        email,
        active: b,
    }
}
