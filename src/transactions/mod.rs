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

#[derive(serde::Deserialize, serde::Serialize, Debug, Copy, PartialEq, Clone)]
pub struct Transaction {
    pub payment_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

impl Transaction {
    #[cfg(test)]
    pub fn new_deposit(client: u16, tx: u32, amount: Option<f32>) -> Self {
        Transaction {
            payment_type: TransactionType::Deposit,
            client,
            tx,
            amount,
        }
    }
    #[cfg(test)]
    pub fn new_withdrawal(client: u16, tx: u32, amount: Option<f32>) -> Self {
        Transaction {
            payment_type: TransactionType::Withdrawal,
            client,
            tx,
            amount,
        }
    }
    #[cfg(test)]
    pub fn new_dispute(client: u16, tx: u32) -> Self {
        Transaction {
            payment_type: TransactionType::Dispute,
            client,
            tx,
            amount: None,
        }
    }
    #[cfg(test)]
    pub fn new_resolve(client: u16, tx: u32) -> Self {
        Transaction {
            payment_type: TransactionType::Resolve,
            client,
            tx,
            amount: None,
        }
    }
    #[cfg(test)]
    pub fn new_chargeback(client: u16, tx: u32) -> Self {
        Transaction {
            payment_type: TransactionType::Chargeback,
            client,
            tx,
            amount: None,
        }
    }
}
