use std::ops::Deref;

pub trait Transaction {
    type Error: std::error::Error + 'static;

    async fn commit(self) -> Result<(), Self::Error>;

    async fn rollback(self) -> Result<(), Self::Error>;
}

pub trait TransactionBuilder {
    type Error: std::error::Error + 'static;
    type Transaction: Transaction;

    async fn begin(&self) -> Result<Self::Transaction, Self::Error>;
}
