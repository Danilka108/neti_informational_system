use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash)]
pub struct User<Id = i32> {
    pub id: Id,
    pub email: Box<str>,
    pub password: Box<[u8]>,
}

impl PartialEq for User<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for User<i32> {}
