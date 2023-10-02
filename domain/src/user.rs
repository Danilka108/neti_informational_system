use serde::{Deserialize, Serialize};

use crate::Role;

#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct User<Id = i32> {
    pub id: Id,
    pub email: String,
    pub password: Vec<u8>,
    pub role: Role,
}

impl PartialEq for User<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for User<i32> {}
