use domain::User;

pub trait UserRepository: Sized {
    type Transaction;
    type Error: std::error::Error + Send + Sync + 'static;

    async fn save(
        &mut self,
        t: &mut Self::Transaction,
        user: domain::User<()>,
    ) -> Result<User, Self::Error>;

    async fn find_by_email(
        &self,
        t: &mut Self::Transaction,
        email: &str,
    ) -> Result<Option<User>, Self::Error>;

    async fn find(&self, t: &mut Self::Transaction, id: i32) -> Result<Option<User>, Self::Error>;
}
