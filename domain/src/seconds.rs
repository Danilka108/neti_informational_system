use std::time::{Duration, SystemTime};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Seconds {
    pub val: u64,
}

#[derive(Serialize, Deserialize, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct SecondsFromUnixEpoch {
    pub seconds: Seconds,
}

impl SecondsFromUnixEpoch {
    pub fn new_expires_at(ttl: Seconds) -> Result<SecondsFromUnixEpoch, anyhow::Error> {
        let duration: Seconds = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("failed to get duration since unix epoch")?
            .checked_add(Duration::from_secs(ttl.val))
            .context("failed to compute issuted at for jwt token")?
            .as_secs()
            .into();

        Ok(duration.into())
    }

    pub fn is_expired(&self) -> Result<bool, anyhow::Error> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("failed to get duration since unix epoch")?
            .as_secs();

        Ok(now > self.seconds.val)
    }
}

impl From<Seconds> for SecondsFromUnixEpoch {
    fn from(seconds: Seconds) -> Self {
        Self { seconds }
    }
}

impl From<u64> for SecondsFromUnixEpoch {
    fn from(val: u64) -> Self {
        Self {
            seconds: Seconds { val },
        }
    }
}

impl From<u64> for Seconds {
    fn from(val: u64) -> Self {
        Self { val }
    }
}
