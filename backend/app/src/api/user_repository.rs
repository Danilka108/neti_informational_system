use domain::User;

use super::Transaction;

pub trait UserRepository: Transaction {
    async fn save(&mut self, user: domain::User<()>) -> Result<User, Self::Error>;

    async fn find_by_email(&mut self, email: &str) -> Result<Option<User>, Self::Error>;
}
