mod read;
use serde::Deserialize;

pub use read::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    payment_type: TransactionType,
    client: u16,
    tx: u32,
    amount: f32,
}
#[derive(Deserialize, Debug)]
pub struct TransactionDetails {
    payment_type: TransactionType,
    tx: u32,
    amount: f32,
}

impl From<Transaction> for TransactionDetails {
    fn from(transaction: Transaction) -> Self {
        TransactionDetails {
            payment_type: transaction.payment_type,
            tx: transaction.tx,
            amount: transaction.amount,
        }
    }
}
