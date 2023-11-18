use argon2::{Algorithm, Version};

#[derive(Debug, Clone, Copy)]
pub struct Argon2Params {
    pub paralelism_degree: u32,
    pub algorithm: Algorithm,
    pub version: Version,
}
