use crate::accounts::ClientAccount;
use crate::transactions::{Transaction, TransactionType};
use std::collections::HashMap;

pub fn generate_client_accounts(transactions: Vec<Transaction>) -> HashMap<u16, ClientAccount> {
    let mut client_accounts: HashMap<u16, ClientAccount> = HashMap::new();

    for transaction in transactions {
        let rounded_transaction = transaction.round();
        let account = client_accounts
            .entry(rounded_transaction.client)
            .or_insert(ClientAccount::new(rounded_transaction.client));

        let client = rounded_transaction.client;

        if transaction.payment_type == TransactionType::Deposit {
            let updated_account = account.clone().deposit_to_account(rounded_transaction);
            client_accounts.insert(client, updated_account);
        } else if transaction.payment_type == TransactionType::Withdrawal {
            let updated_account = account.clone().withdraw_from_account(rounded_transaction);
            client_accounts.insert(client, updated_account);
        } else if transaction.payment_type == TransactionType::Dispute {
            let updated_account = account.clone().dispute_to_account(rounded_transaction);
            client_accounts.insert(client, updated_account);
        } else if transaction.payment_type == TransactionType::Resolve {
            let updated_account = account.clone().resolve_to_account(rounded_transaction);
            client_accounts.insert(client, updated_account);
        } else if transaction.payment_type == TransactionType::Chargeback {
            let updated_account = account.clone().chargeback_to_account(rounded_transaction);
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
    use rust_decimal_macros::dec;

    #[rstest()]
    fn test_basic() {
        let mut transactions = vec![];
        transactions.push(Transaction::new_deposit(1, 1, Some(dec!(1.0))));
        transactions.push(Transaction::new_deposit(2, 2, Some(dec!(2.0))));
        transactions.push(Transaction::new_deposit(1, 3, Some(dec!(2.0))));
        transactions.push(Transaction::new_withdrawal(1, 4, Some(dec!(1.5))));
        transactions.push(Transaction::new_withdrawal(2, 5, Some(dec!(3.0))));

        let result = generate_client_accounts(transactions);

        insta::assert_json_snapshot!(serde_json::json!(result));
    }

    #[rstest()]
    fn test_extra_decimal_points() {
        let mut transactions = vec![];
        transactions.push(Transaction::new_deposit(1, 1, Some(dec!(1.12345678))));
        transactions.push(Transaction::new_deposit(2, 2, Some(dec!(2.12345678))));
        transactions.push(Transaction::new_deposit(1, 3, Some(dec!(2.12345678))));
        transactions.push(Transaction::new_withdrawal(1, 4, Some(dec!(2.12345678))));
        transactions.push(Transaction::new_withdrawal(2, 5, Some(dec!(2.12345678))));

        let result = generate_client_accounts(transactions);

        insta::assert_json_snapshot!(serde_json::json!(result));
    }

    #[rstest(
    transactions,
    case::unresolved_dispute(
        vec![Transaction::new_deposit(1, 1, Some(dec!(1.0))),
        Transaction::new_deposit(2, 2, Some(dec!(2.0))),
        Transaction::new_deposit(1, 3, Some(dec!(2.0))),
        Transaction::new_withdrawal(1, 4, Some(dec!(1.5))),
        Transaction::new_withdrawal(2, 5, Some(dec!(3.0))),
        Transaction::new_dispute(1,1)]),
    case::resolved_dispute(
        vec![Transaction::new_deposit(1, 1, Some(dec!(1.0))),
        Transaction::new_deposit(2, 2, Some(dec!(2.0))),
        Transaction::new_deposit(1, 3, Some(dec!(2.0))),
        Transaction::new_withdrawal(1, 4, Some(dec!(1.5))),
        Transaction::new_withdrawal(2, 5, Some(dec!(3.0))),
        Transaction::new_dispute(1,1),
        Transaction::new_resolve(1,1)]),
    case::chargeback(
        vec![Transaction::new_deposit(1, 1, Some(dec!(1.0))),
        Transaction::new_deposit(2, 2, Some(dec!(2.0))),
        Transaction::new_deposit(1, 3, Some(dec!(2.0))),
        Transaction::new_withdrawal(1, 4, Some(dec!(1.5))),
        Transaction::new_withdrawal(2, 5, Some(dec!(3.0))),
        Transaction::new_dispute(1,1),
        Transaction::new_chargeback(1,1)])
    )]
    fn test_dispute_flow(transactions: Vec<Transaction>) {
        let result = generate_client_accounts(transactions);

        insta::assert_json_snapshot!(serde_json::json!(result));
    }
}
