use crate::models::{User, Payment, RecurringPayment};
use crate::error::PaymentError;
use stellar_sdk::Server;
use stellar_sdk::Keypair;
use stellar_sdk::types::Operation;
use stellar_sdk::types::{
    Account,
    Asset,
};
use stellar_sdk::types::Transaction;
use soroban_sdk::xdr::MuxedAccount;
use stellar_sdk::Network;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct PaymentSystem {
    users: HashMap<String, User>,
    payments: Vec<Payment>,
    recurring_payments: Vec<RecurringPayment>,
    server: Server,
}

impl PaymentSystem {
    pub fn new() -> Self {
        PaymentSystem {
            users: HashMap::new(),
            payments: Vec::new(),
            recurring_payments: Vec::new(),
            server: Server::new("https://horizon-testnet.stellar.org".to_string()).unwrap(),
        }
    }

    pub fn create_user(&mut self, public_key: &str, secret_key: &str) -> Result<(), PaymentError> {
        let keypair = Keypair::from_secret(secret_key).map_err(|_| PaymentError::InvalidKey)?;
        let user = User {
            keypair,
            balance: 0.0,
        };
        self.users.insert(public_key.to_string(), user);
        Ok(())
    }

    pub async fn send_payment(&mut self, from: &str, to: &str, amount: f64, message: &str) -> Result<(), PaymentError> {
        let sender = self.users.get_mut(from).ok_or(PaymentError::UserNotFound)?;
        if sender.balance < amount {
            return Err(PaymentError::InsufficientBalance);
        }

        let destination = Keypair::from_public_key(to).map_err(|_| PaymentError::InvalidKey)?;
        let source_account = self.server.load_account(&sender.keypair).await.map_err(|_| PaymentError::TransactionFailed)?;

        let transaction = Transaction::new(&source_account)
            .add_operation(Operation::payment(
                destination.public_key(),
                Asset::native(),
                &amount.to_string(),
            ))
            .set_memo(Some(message.into()))
            .build();

        let result = self.server.submit_transaction(&transaction).await;

        if result.is_ok() {
            sender.balance -= amount;
            if let Some(receiver) = self.users.get_mut(to) {
                receiver.balance += amount;
            }

            self.payments.push(Payment {
                from: from.to_string(),
                to: to.to_string(),
                amount,
                message: message.to_string(),
                timestamp: Utc::now(),
            });

            Ok(())
        } else {
            Err(PaymentError::TransactionFailed)
        }
    }

    pub fn get_balance(&self, public_key: &str) -> Result<f64, PaymentError> {
        let user = self.users.get(public_key).ok_or(PaymentError::UserNotFound)?;
        Ok(user.balance)
    }

    pub fn add_recurring_payment(&mut self, from: &str, to: &str, amount: f64, message: &str, interval_days: i64) {
        let recurring_payment = RecurringPayment {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            message: message.to_string(),
            interval: chrono::Duration::days(interval_days),
            next_payment: Utc::now() + chrono::Duration::days(interval_days),
        };
        self.recurring_payments.push(recurring_payment);
    }

    pub async fn process_recurring_payments(&mut self) -> Result<(), PaymentError> {
        let now = Utc::now();
        for payment in &mut self.recurring_payments {
            if payment.next_payment <= now {
                self.send_payment(&payment.from, &payment.to, payment.amount, &payment.message).await?;
                payment.next_payment = now + payment.interval;
            }
        }
        Ok(())
    }

    pub fn get_transaction_history(&self, public_key: &str) -> Vec<&Payment> {
        self.payments
            .iter()
            .filter(|payment| payment.from == public_key || payment.to == public_key)
            .collect()
    }
}