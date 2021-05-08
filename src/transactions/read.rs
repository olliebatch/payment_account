use std::fs::File;

use crate::config::Config;
use crate::transactions::Transaction;
use crate::BoxError;

pub fn read_transaction_file(config: &Config) -> Result<Vec<Transaction>, BoxError> {
    let file = File::open(&config.transaction_file)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut transactions: Vec<Transaction> = vec![];
    for result in rdr.records() {
        let record = result?;
        let transaction: Transaction = record.deserialize(None)?;
        transactions.push(transaction);
    }

    Ok(transactions)
}
