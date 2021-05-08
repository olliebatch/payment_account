use crate::transactions::Transaction;

pub struct ClientAccount {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
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
        }
    }

    #[cfg(test)]
    pub fn new_for_testing(client_id: u16, balance: f32) -> Self {
        ClientAccount {
            client: client_id,
            available: balance,
            held: 0.0,
            total: balance,
            locked: false,
        }
    }

    pub fn deposit_to_account(self, transaction: Transaction) -> Self {
        let new_avail = self.available + transaction.amount;
        let total = self.total + transaction.amount;
        ClientAccount {
            available: new_avail,
            total,
            ..self
        }
    }
    pub fn withdraw_from_account(self, transaction: Transaction) -> Self {
        let new_avail = self.available - transaction.amount;
        let total = self.total - transaction.amount;
        ClientAccount {
            available: new_avail,
            total,
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
}
