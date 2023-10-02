pub trait PasswordEncoder {
    fn encode(&self, plain_password: &str) -> Vec<u8>;

    fn is_matches(&self, plain_password: &str, encoded_password: &[u8]) -> bool;
}
