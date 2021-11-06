use crate::day3::user::User;

pub fn print_user() {
    println!("{:?}", User {
        id: 1,
        email: String::from("webmaster@neilwang.wiki"),
        username: String::from("Neil"),
        active: true,
    });
}
