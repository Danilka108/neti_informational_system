use anyhow::Context;
use argon2::{Algorithm, Argon2, Params, ParamsBuilder, Version};
use password_hash::{PasswordHasher as ExternalPasswordHasher, PasswordVerifier, SaltString};
use rand::{rngs::StdRng, SeedableRng};

use app::hasher::{HashedPassword, PasswordHasher};

#[derive(Debug, Clone, Copy)]
pub struct Argon2Params {
    pub paralelism_degree: u32,
    pub algorithm: Algorithm,
    pub version: Version,
}

#[derive(Debug, Clone)]
pub struct Argon2PasswordHasher {
    params: Params,
    algorithm: Algorithm,
    version: Version,
}

impl Argon2PasswordHasher {
    pub(crate) fn new(
        Argon2Params {
            paralelism_degree,
            algorithm,
            version,
        }: Argon2Params,
    ) -> Self {
        let params = ParamsBuilder::new()
            .p_cost(paralelism_degree)
            .build()
            .unwrap();
        // .context("failed to build argon2 params")?;

        Self {
            params,
            algorithm,
            version,
        }
    }
}

#[async_trait::async_trait]
impl PasswordHasher for Argon2PasswordHasher {
    async fn hash(&self, password: String) -> Result<app::user::HashedPassword, anyhow::Error> {
        let context = Argon2::new(self.algorithm, self.version, self.params.clone());

        let hashed_password = tokio::task::spawn_blocking(move || {
            let rng = &mut StdRng::from_entropy();
            let salt = SaltString::generate(rng);

            context
                .hash_password(password.as_bytes(), &salt)
                .map(|v| v.to_string())
        })
        .await
        .context("failed to join blocking task")??;

        Ok(HashedPassword {
            value: hashed_password,
        })
    }

    async fn is_matches(
        &self,
        password: &str,
        HashedPassword {
            value: hashed_password,
        }: &app::user::HashedPassword,
    ) -> Result<bool, anyhow::Error> {
        let context = Argon2::new(self.algorithm, self.version, self.params.clone());

        let password = password.to_owned();
        let hashed_password = hashed_password.to_owned();

        let verify_res = tokio::task::spawn_blocking(move || {
            let hash = password_hash::PasswordHash::try_from(hashed_password.as_str());
            match hash {
                Ok(hash) => Ok(context.verify_password(password.as_bytes(), &hash)),
                Err(err) => Err(err),
            }
        })
        .await
        .context("failed to join blocking task")?
        .context("invalid hashed password value")?;

        match verify_res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}
