use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Session {
    pub user_id: i32,
    pub metadata: String,
    pub refresh_token: String,
    pub ttl_in_seconds: u64,
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.user_id.eq(&other.user_id) && self.metadata.eq(&other.metadata)
    }
}

impl Eq for Session {}
