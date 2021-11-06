use crate::day3::user::User;

pub fn struct_method(id: i32, username: String, email: String, b: bool) -> i32 {
    User {
        id,
        username,
        email,
        active: b
    }.get_id()
}
