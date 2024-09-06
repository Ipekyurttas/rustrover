use stellar_sdk::Keypair;
use chrono::{DateTime, Utc};

pub struct User {
    pub keypair: Keypair,
    pub balance: f64,
}

pub struct Payment {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

pub struct RecurringPayment {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub message: String,
    pub interval: chrono::Duration,
    pub next_payment: DateTime<Utc>,
}