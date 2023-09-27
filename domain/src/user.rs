use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash)]
pub struct User {
    pub id: u32,
    pub email: Box<str>,
    pub password_encoded: Box<str>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for User {}
