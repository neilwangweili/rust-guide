use crate::day3::user::User;

pub fn copy_struct(id: i32, username: String, email: String, b: bool) -> User {
    let user1 = User {
        id,
        username,
        email,
        active: b,
    };
    User {
        active: user1.active,
        ..user1
    }
}
