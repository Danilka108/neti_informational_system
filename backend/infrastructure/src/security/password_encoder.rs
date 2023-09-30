use ring::pbkdf2::{self, PBKDF2_HMAC_SHA512};

use crate::env_config::EnvConfig;

pub struct Pbkdf2PassEncoder<'cfg> {
    env_config: &'cfg EnvConfig,
}

impl<'cfg> app::api::PasswordEncoder for Pbkdf2PassEncoder<'cfg> {
    fn encode(&self, plain_password: &str) -> Box<[u8]> {
        let mut out = [0u8; ring::digest::SHA512_OUTPUT_LEN];

        pbkdf2::derive(
            PBKDF2_HMAC_SHA512,
            self.env_config.pbkdf2_iters_count,
            &self.env_config.pbkdf2_salt,
            plain_password.as_bytes(),
            &mut out,
        );

        out.into()
    }

    fn is_matches(&self, plain_password: &str, encoded_password: &[u8]) -> bool {
        pbkdf2::verify(
            PBKDF2_HMAC_SHA512,
            self.env_config.pbkdf2_iters_count,
            &self.env_config.pbkdf2_salt,
            plain_password.as_bytes(),
            encoded_password,
        )
        .is_ok()
    }
}
