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
pub struct Transaction {
    payment_type: TransactionType,
    client: u16,
    tx: u32,
    amount: f32,
}
