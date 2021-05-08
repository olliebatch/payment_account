mod generate;

pub use generate::*;

use crate::transactions::Transaction;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub struct ClientAccount {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
    transactions: Vec<Transaction>,
}

impl ClientAccount {
    pub fn new(client_id: u16) -> Self {
        // start a new account with 0 balance and unlocked
        ClientAccount {
            client: client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            transactions: vec![],
        }
    }

    fn calc_available(&self) -> f32 {
        self.total - self.held
    }

    #[cfg(test)]
    pub fn new_for_testing(client_id: u16, balance: f32) -> Self {
        ClientAccount {
            client: client_id,
            available: balance,
            held: 0.0,
            total: balance,
            locked: false,
            transactions: vec![],
        }
    }

    pub fn deposit_to_account(mut self, transaction: Transaction) -> Self {
        if self.is_locked() {
            return self;
        }
        let new_avail = self.available + transaction.amount;
        let total = self.total + transaction.amount;
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
        if transaction.amount > self.available {
            return self;
        }
        let new_avail = self.available - transaction.amount;
        let total = self.total - transaction.amount;
        self.transactions.push(transaction);
        ClientAccount {
            available: new_avail,
            total,
            ..self
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

    #[rstest()]
    fn add_deposit_test() {
        let client_account = ClientAccount::new(1);
        let transaction = Transaction::new_deposit(1, 1, 1.5);

        let updated = client_account.deposit_to_account(transaction);
        assert_eq!(updated.available, 1.5);
        assert_eq!(updated.total, 1.5)
    }

    #[rstest()]
    fn withdraw_funds_test() {
        let client_account = ClientAccount::new_for_testing(1, 5.0);
        let transaction = Transaction::new_withdrawal(1, 1, 1.5);

        let updated = client_account.withdraw_from_account(transaction);
        assert_eq!(updated.available, 3.5);
        assert_eq!(updated.total, 3.5)
    }

    #[rstest()]
    fn withdraw_funds_more_than_avail() {
        let client_account = ClientAccount::new_for_testing(1, 2.0);
        let transaction = Transaction::new_withdrawal(1, 1, 3.0);

        let updated = client_account.withdraw_from_account(transaction);
        assert_eq!(updated.available, 2.0);
        assert_eq!(updated.total, 2.0)
    }

    #[rstest()]
    fn lock_account() {
        let client_account = ClientAccount::new_for_testing(1, 2.0);

        let account = client_account.lock_account();
        assert_eq!(account.locked, true);
    }
}
