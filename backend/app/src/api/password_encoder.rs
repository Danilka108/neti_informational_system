pub trait PasswordEncoder {
    fn encode(&self, plain_password: &str) -> Box<[u8]>;

    fn is_matches(&self, plain_password: &str, encoded_password: &[u8]) -> bool;
}
