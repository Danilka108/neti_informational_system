use async_trait::async_trait;
use domain::User;

#[async_trait]
pub trait UserRepository {
    type Transaction;

    async fn insert(
        &mut self,
        t: &mut Self::Transaction,
        user: domain::User<()>,
    ) -> Result<Option<User>, anyhow::Error>;

    async fn update(
        &mut self,
        t: &mut Self::Transaction,
        user: domain::User<()>,
    ) -> Result<Option<User>, anyhow::Error>;

    async fn find_by_email(
        &self,
        t: &mut Self::Transaction,
        email: &str,
    ) -> Result<Option<User>, anyhow::Error>;

    async fn find_by_id(
        &self,
        t: &mut Self::Transaction,
        id: i32,
    ) -> Result<Option<User>, anyhow::Error>;
}
