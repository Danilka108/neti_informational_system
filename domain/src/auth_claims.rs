use serde::{Deserialize, Serialize};

use crate::{Role, SecondsFromUnixEpoch};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct AuthClaims {
    pub user_id: i32,
    pub email: String,
    pub expires_at: SecondsFromUnixEpoch,
    pub role: Role,
}
