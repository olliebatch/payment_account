use payment_account::{config::Config, transactions};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new().expect("Can't load args provided");

    let transactions = transactions::read_transaction_file(&config);
    println!("{:?}", transactions);
    Ok(())
}
