use std::num::NonZeroU32;

use ring::pbkdf2::{self, PBKDF2_HMAC_SHA512};

use crate::env_config::EnvConfig;

pub struct Pbkdf2PassEncoder {
    iters_count: NonZeroU32,
    salt: [u8; ring::digest::SHA512_OUTPUT_LEN],
}

impl Pbkdf2PassEncoder {
    pub fn new(config: &EnvConfig) -> Self {
        Self {
            iters_count: config.pbkdf2_iters_count,
            salt: config.pbkdf2_salt.clone(),
        }
    }
}

impl app::api::PasswordEncoder for Pbkdf2PassEncoder {
    fn encode(&self, plain_password: &str) -> Vec<u8> {
        let mut out = [0u8; ring::digest::SHA512_OUTPUT_LEN];

        pbkdf2::derive(
            PBKDF2_HMAC_SHA512,
            self.iters_count,
            &self.salt,
            plain_password.as_bytes(),
            &mut out,
        );

        out.into()
    }

    fn is_matches(&self, plain_password: &str, encoded_password: &[u8]) -> bool {
        pbkdf2::verify(
            PBKDF2_HMAC_SHA512,
            self.iters_count,
            &self.salt,
            plain_password.as_bytes(),
            encoded_password,
        )
        .is_ok()
    }
}
