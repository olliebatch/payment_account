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
        } else if transaction.payment_type == TransactionType::Dispute {
            let updated_account = account.clone().dispute_to_account(transaction);
            client_accounts.insert(client, updated_account);
        }
    }
    client_accounts
}

#[cfg(test)]
mod tests {

    use crate::accounts::generate_client_accounts;
    use crate::transactions::Transaction;
    use rstest::rstest;

    #[rstest()]
    fn test_basic() {
        let mut transactions = vec![];
        transactions.push(Transaction::new_deposit(1, 1, Some(1.0)));
        transactions.push(Transaction::new_deposit(2, 2, Some(2.0)));
        transactions.push(Transaction::new_deposit(1, 3, Some(2.0)));
        transactions.push(Transaction::new_withdrawal(1, 4, Some(1.5)));
        transactions.push(Transaction::new_withdrawal(2, 5, Some(3.0)));

        let result = generate_client_accounts(transactions);

        insta::assert_json_snapshot!(serde_json::json!(result));
    }
}
