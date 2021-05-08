use std::fs::File;

use crate::config::Config;
use crate::transactions::{Transaction, TransactionDetails};
use crate::BoxError;
use std::collections::HashMap;

pub fn read_transaction_file(
    config: &Config,
) -> Result<HashMap<u16, TransactionDetails>, BoxError> {
    let file = File::open(&config.transaction_file)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut transactions: HashMap<u16, TransactionDetails> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let transaction: Transaction = record.deserialize(None)?;
        transactions.insert(transaction.client, transaction.into());
    }

    println!("{:?}", transactions);
    Ok(transactions)
}
