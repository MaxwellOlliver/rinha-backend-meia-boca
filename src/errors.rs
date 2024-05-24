use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Wallet not found")]
    WalletNotFound,
    #[error("Database error")]
    DbError(#[from] sea_orm::DbErr),
}
