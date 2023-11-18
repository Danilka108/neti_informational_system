#[derive(Clone)]
pub struct JwtKeys(pub jsonwebtoken::EncodingKey, pub jsonwebtoken::DecodingKey);

impl std::fmt::Debug for JwtKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("JwtKeys").field(&"?").finish()
    }
}
