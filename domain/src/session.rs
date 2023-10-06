use serde::{Deserialize, Serialize};

use crate::SecondsFromUnixEpoch;

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Session {
    pub user_id: i32,
    /// max len = 500
    pub metadata: String,
    pub refresh_token: String,
    pub expires_at: SecondsFromUnixEpoch,
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.user_id.eq(&other.user_id) && self.metadata.eq(&other.metadata)
    }
}

impl Eq for Session {}
