pub mod models;
pub mod payment_system;
pub mod error;

use payment_system::PaymentSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = PaymentSystem::new();

    // Örnek kullanım
    system.create_user("user1_public_key", "user1_secret_key")?;
    system.create_user("user2_public_key", "user2_secret_key")?;

    system.send_payment("user1_public_key", "user2_public_key", 10.0, "İlk ödeme").await?;

    system.add_recurring_payment("user1_public_key", "user2_public_key", 5.0, "Aylık ödeme", 30);

    // Düzenli ödemeleri işle
    system.process_recurring_payments().await?;

    // Bakiye sorgulama
    let balance = system.get_balance("user1_public_key")?;
    println!("User1 bakiyesi: {}", balance);

    // İşlem geçmişi görüntüleme
    let history = system.get_transaction_history("user1_public_key");
    for payment in history {
        println!("From: {}, To: {}, Amount: {}, Message: {}", payment.from, payment.to, payment.amount, payment.message);
    }

    Ok(())
}
