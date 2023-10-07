use std::num::NonZeroU32;

use serde::de::Unexpected;

#[derive(Debug, serde::Deserialize)]
pub struct EnvConfig {
    // Postgresql
    pub pg_dbname: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_host: String,
    pub pg_port: u16,
    #[serde(default = "get_default_workers_count")]
    pub pg_max_connections: u32,
    // Redis
    pub redis_uri: String,
    pub redis_max_connections: u32,
    // PBKDF2 password encoding
    pub pbkdf2_iters_count: NonZeroU32,
    #[serde(deserialize_with = "parse_as_bytes_sequence")]
    pub pbkdf2_salt: [u8; ring::digest::SHA512_OUTPUT_LEN],
    // Tokens and Session
    pub refresh_token_length: usize,
    pub jwt_secret: String,
    pub jwt_token_ttl_in_seconds: u64,
    pub sessions_max_number_per_user: usize,
    pub session_ttl_in_seconds: u64,
    // Other
    #[serde(default = "get_default_workers_count")]
    pub workers_count: u32,
    pub sqlx_log: Option<log::LevelFilter>,
    pub application_name: String,
}

fn parse_as_bytes_sequence<'de, D: serde::Deserializer<'de>, const LEN: usize>(
    deserializer: D,
) -> Result<[u8; LEN], D::Error> {
    struct Visitor<const LEN: usize>;

    impl<'de, const LEN: usize> serde::de::Visitor<'de> for Visitor<LEN> {
        type Value = [u8; LEN];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                formatter,
                "an array of {0} bytes in binary representation",
                LEN,
            )
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let mut v = v.chars();
            let mut bytes = [0; LEN];

            'l: for i in 0..LEN {
                let mut n = 0u8;

                for j in (0..=7).rev() {
                    n &= match v.next() {
                        Some('0') => 0b0,
                        Some('1') => 0b1,
                        Some(s) => return Err(E::invalid_value(Unexpected::Char(s), &self)),
                        None if j != 7 => return Err(E::invalid_length(j + 1, &self)),
                        None => break 'l,
                    } << j;
                }

                bytes[i] = n;
            }

            Ok(bytes)
        }
    }
    deserializer.deserialize_string(Visitor::<LEN>)
}

fn get_default_workers_count() -> u32 {
    std::thread::available_parallelism().unwrap().get() as u32
}

impl EnvConfig {
    pub fn try_load() -> Result<Self, LoadEnvConfigError> {
        envy::from_env().map_err(LoadEnvConfigError)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("failed to load env config, cause {}", .0)]
pub struct LoadEnvConfigError(#[from] envy::Error);
