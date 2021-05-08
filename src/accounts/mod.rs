mod generate;
mod write;

pub use generate::*;
pub use write::*;

use crate::transactions::{Transaction, TransactionType};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub struct ExternalClientAccount {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

impl From<&ClientAccount> for ExternalClientAccount {
    fn from(client: &ClientAccount) -> Self {
        ExternalClientAccount {
            client: client.client,
            available: client.available,
            held: client.held,
            total: client.total,
            locked: client.locked,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub struct ClientAccount {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
    transactions: Vec<Transaction>,
}

impl ClientAccount {
    pub fn new(client_id: u16) -> Self {
        // start a new account with 0 balance and unlocked
        ClientAccount {
            client: client_id,
            available: dec!(0.0),
            held: dec!(0.0),
            total: dec!(0.0),
            locked: false,
            transactions: vec![],
        }
    }

    #[cfg(test)]
    pub fn new_for_testing(client_id: u16, balance: Decimal) -> Self {
        ClientAccount {
            client: client_id,
            available: balance,
            held: dec!(0.0),
            total: balance,
            locked: false,
            transactions: vec![],
        }
    }

    pub fn deposit_to_account(mut self, transaction: Transaction) -> Self {
        if self.is_locked() {
            return self;
        }
        // assume safe to unwrap as deposit
        let amount = transaction.amount.unwrap();
        let new_avail = self.available + amount;
        let total = self.total + amount;

        self.transactions.push(transaction);
        ClientAccount {
            available: new_avail,
            total,
            transactions: self.transactions,
            ..self
        }
    }

    pub fn withdraw_from_account(mut self, transaction: Transaction) -> Self {
        if self.is_locked() {
            return self;
        }
        // assume that an amount is always provided for a withdrawal
        let amount = transaction.amount.unwrap();

        if amount > self.available {
            return self;
        }
        let new_avail = self.available - amount;
        let total = self.total - amount;
        self.transactions.push(transaction);
        ClientAccount {
            available: new_avail,
            total,
            ..self
        }
    }

    fn find_valid_transaction(&self, trx_id: u32) -> Option<&Transaction> {
        self.transactions.iter().find(|&transaction| {
            transaction.tx == trx_id
                && (transaction.payment_type == TransactionType::Withdrawal
                    || transaction.payment_type == TransactionType::Deposit)
        })
    }

    pub fn dispute_to_account(mut self, transaction: Transaction) -> Self {
        let existing_trx = self.find_valid_transaction(transaction.tx);

        if let Some(trx) = existing_trx {
            let trx_amount = trx.amount.unwrap();
            let new_avail = self.available - trx_amount;
            let held_funds = self.held + trx_amount;
            self.transactions.push(transaction);
            return ClientAccount {
                available: new_avail,
                held: held_funds,
                transactions: self.transactions,
                ..self
            };
        } else {
            return self;
        }
    }

    pub fn resolve_to_account(mut self, transaction: Transaction) -> Self {
        let existing_trx = self.find_valid_transaction(transaction.tx);

        if let Some(trx) = existing_trx {
            let trx_amount = trx.amount.unwrap();
            let new_avail = self.available + trx_amount;
            let held_funds = self.held - trx_amount;
            self.transactions.push(transaction);
            return ClientAccount {
                available: new_avail,
                held: held_funds,
                transactions: self.transactions,
                ..self
            };
        } else {
            return self;
        }
    }

    pub fn chargeback_to_account(mut self, transaction: Transaction) -> Self {
        let existing_trx = self.find_valid_transaction(transaction.tx);

        if let Some(trx) = existing_trx {
            let trx_amount = trx.amount.unwrap();
            let held_funds = self.held - trx_amount;
            let total = self.total - trx_amount;
            self.transactions.push(transaction);
            let account = ClientAccount {
                total,
                held: held_funds,
                transactions: self.transactions,
                ..self
            };
            account.lock_account()
        } else {
            return self;
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn lock_account(self) -> Self {
        ClientAccount {
            locked: true,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::ClientAccount;
    use crate::transactions::Transaction;
    use rstest::rstest;
    use rust_decimal_macros::dec;

    #[rstest()]
    fn add_deposit_test() {
        let client_account = ClientAccount::new(1);
        let transaction = Transaction::new_deposit(1, 1, Some(dec!(1.5)));

        let updated = client_account.deposit_to_account(transaction);
        assert_eq!(updated.available, dec!(1.5));
        assert_eq!(updated.total, dec!(1.5))
    }

    #[rstest()]
    fn withdraw_funds_test() {
        let client_account = ClientAccount::new_for_testing(1, dec!(5.0));
        let transaction = Transaction::new_withdrawal(1, 1, Some(dec!(1.5)));

        let updated = client_account.withdraw_from_account(transaction);
        assert_eq!(updated.available, dec!(3.5));
        assert_eq!(updated.total, dec!(3.5))
    }

    #[rstest()]
    fn withdraw_funds_more_than_avail() {
        let client_account = ClientAccount::new_for_testing(1, dec!(2.0));
        let transaction = Transaction::new_withdrawal(1, 1, Some(dec!(3.0)));

        let updated = client_account.withdraw_from_account(transaction);
        assert_eq!(updated.available, dec!(2.0));
        assert_eq!(updated.total, dec!(2.0))
    }

    #[rstest()]
    fn dispute_funds_avail() {
        let client_account = ClientAccount::new_for_testing(1, dec!(2.0));
        let transaction = Transaction::new_deposit(1, 1, Some(dec!(3.0)));
        let dispute = Transaction::new_dispute(1, 1);
        let updated = client_account.deposit_to_account(transaction);
        let disputed = updated.dispute_to_account(dispute);

        assert_eq!(disputed.available, dec!(2.0));
        assert_eq!(disputed.held, dec!(3.0));
        assert_eq!(disputed.total, dec!(5.0))
    }
    #[rstest()]
    fn resolve_funds_avail() {
        let client_account = ClientAccount::new_for_testing(1, dec!(2.0));
        let transaction = Transaction::new_deposit(1, 1, Some(dec!(3.0)));
        let dispute = Transaction::new_dispute(1, 1);
        let resolve = Transaction::new_resolve(1, 1);
        let updated = client_account.deposit_to_account(transaction);
        let disputed = updated.dispute_to_account(dispute);
        let resolved = disputed.resolve_to_account(resolve);

        assert_eq!(resolved.available, dec!(5.0));
        assert_eq!(resolved.held, dec!(0.0));
        assert_eq!(resolved.total, dec!(5.0))
    }

    #[rstest()]
    fn chargeback_lock_account() {
        let client_account = ClientAccount::new_for_testing(1, dec!(2.0));
        let transaction = Transaction::new_deposit(1, 1, Some(dec!(3.0)));
        let dispute = Transaction::new_dispute(1, 1);
        let chargeback_trx = Transaction::new_chargeback(1, 1);
        let updated = client_account.deposit_to_account(transaction);
        let disputed = updated.dispute_to_account(dispute);
        let chargeback = disputed.chargeback_to_account(chargeback_trx);

        assert_eq!(chargeback.available, dec!(2.0));
        assert_eq!(chargeback.held, dec!(0.0));
        assert_eq!(chargeback.total, dec!(2.0));
        assert_eq!(chargeback.locked, true);
    }

    #[rstest()]
    fn lock_account() {
        let client_account = ClientAccount::new_for_testing(1, dec!(2.0));

        let account = client_account.lock_account();
        assert_eq!(account.locked, true);
    }
}
