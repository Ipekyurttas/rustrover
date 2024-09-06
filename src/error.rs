use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("User not found")]
    UserNotFound,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Transaction failed")]
    TransactionFailed,
}