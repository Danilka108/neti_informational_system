use std::time::{Duration, SystemTime};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Seconds {
    value: u64,
}

#[derive(Serialize, Deserialize, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct SecondsFromUnixEpoch {
    value: Seconds,
}

impl SecondsFromUnixEpoch {
    pub fn new_expires_at(ttl: Seconds) -> Result<SecondsFromUnixEpoch, anyhow::Error> {
        let duration: Seconds = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("failed to get duration since unix epoch")?
            .checked_add(Duration::from_secs(*ttl.as_ref()))
            .context("failed to compute issuted at for jwt token")?
            .as_secs()
            .into();

        Ok(duration.into())
    }

    pub fn is_expired(&self) -> Result<bool, anyhow::Error> {
        let now: Seconds = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("failed to get duration since unix epoch")?
            .as_secs()
            .into();

        Ok(now > self.value)
    }
}

impl From<Seconds> for SecondsFromUnixEpoch {
    fn from(value: Seconds) -> Self {
        Self { value }
    }
}

impl From<u64> for Seconds {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl AsRef<u64> for Seconds {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl AsRef<Seconds> for SecondsFromUnixEpoch {
    fn as_ref(&self) -> &Seconds {
        &self.value
    }
}

impl AsRef<u64> for SecondsFromUnixEpoch {
    fn as_ref(&self) -> &u64 {
        &self.value.as_ref()
    }
}
