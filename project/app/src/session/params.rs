use super::Seconds;

#[derive(Debug, Clone, Copy)]
pub struct SessionTTL(pub Seconds);

#[derive(Debug, Clone, Copy)]
pub struct SessionsMaxNumber(pub usize);
