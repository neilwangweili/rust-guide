#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub active: bool,
}

impl User {
    pub fn get_id(&self) -> i32 {
        (&self).id
    }
}

