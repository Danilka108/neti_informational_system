use domain::Session;

pub trait SessionRepository {
    type Transaction;
    type Error: std::error::Error + Send + Sync + 'static;

    async fn count_by_user_id(
        &self,
        t: &mut Self::Transaction,
        user_id: i32,
    ) -> Result<usize, Self::Error>;

    async fn find(
        &self,
        t: &mut Self::Transaction,
        user_id: i32,
        metadata: &str,
    ) -> Result<Option<Session>, Self::Error>;

    async fn find_by_metadata_and_token(
        &self,
        t: &mut Self::Transaction,
        refresh_token: &str,
        metadata: &str,
    ) -> Result<Option<Session>, Self::Error>;

    async fn save(
        &mut self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Session, Self::Error>;

    async fn delete(
        &mut self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Session, Self::Error>;
}
