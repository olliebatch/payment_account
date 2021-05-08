use payment_account::{accounts, config::Config, transactions, BoxError};

fn main() -> Result<(), BoxError> {
    let config = Config::new().expect("Can't load args provided");

    let transactions = transactions::read_transaction_file(&config)?;

    let client_accounts = accounts::generate_client_accounts(transactions);

    accounts::write_to_stdout(client_accounts)?;
    Ok(())
}
