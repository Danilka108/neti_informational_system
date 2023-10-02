use serde::{Deserialize, Serialize};

use crate::Role;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct AuthClaims {
    pub user_id: i32,
    pub issued_at_unix_epoch_secs: u64,
    pub role: Role,
}
