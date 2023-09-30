use std::marker::PhantomData;

use sqlx::{Pool, Postgres};

pub struct PgTransactionBuilder<R> {
    conn: Pool<Postgres>,
    _r: PhantomData<R>,
}

impl<R> app::api::TransactionBuilder for PgTransactionBuilder<R>
where
    R: app::api::Transaction + From<sqlx::Transaction<'static, Postgres>>,
{
    type Error = sqlx::Error;
    type Transaction = R;

    async fn begin(&self) -> Result<Self::Transaction, Self::Error> {
        Ok(R::from(self.conn.begin().await?))
    }
}
