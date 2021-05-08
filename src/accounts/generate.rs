use crate::accounts::ClientAccount;
use crate::transactions::{Transaction, TransactionType};
use std::collections::HashMap;

pub fn generate_client_accounts(transactions: Vec<Transaction>) -> HashMap<u16, ClientAccount> {
    let mut client_accounts: HashMap<u16, ClientAccount> = HashMap::new();

    for transaction in transactions {
        let account = client_accounts
            .entry(transaction.client)
            .or_insert(ClientAccount::new(transaction.client));

        let client = transaction.client;

        if transaction.payment_type == TransactionType::Deposit {
            let updated_account = account.clone().deposit_to_account(transaction);
            client_accounts.insert(client, updated_account);
        } else if transaction.payment_type == TransactionType::Withdrawal {
            let updated_account = account.clone().withdraw_from_account(transaction);
            client_accounts.insert(client, updated_account);
        }
    }
    client_accounts
}
