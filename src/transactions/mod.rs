mod read;

pub use read::*;
use rust_decimal::Decimal;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Debug, PartialEq, Clone)]
pub struct Transaction {
    pub payment_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}

impl Transaction {
    pub fn round(self) -> Self {
        if let Some(amount) = self.amount {
            let rounded = amount.round_dp(4);
            Transaction {
                amount: Some(rounded),
                ..self
            }
        } else {
            self
        }
    }

    #[cfg(test)]
    pub fn new_deposit(client: u16, tx: u32, amount: Option<Decimal>) -> Self {
        Transaction {
            payment_type: TransactionType::Deposit,
            client,
            tx,
            amount,
        }
    }
    #[cfg(test)]
    pub fn new_withdrawal(client: u16, tx: u32, amount: Option<Decimal>) -> Self {
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
