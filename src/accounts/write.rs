use crate::accounts::{ClientAccount, ExternalClientAccount};
use crate::BoxError;
use csv::Writer;
use std::collections::HashMap;
use std::io;

pub fn write_to_stdout(accounts: HashMap<u16, ClientAccount>) -> Result<(), BoxError> {
    let mut wtr = Writer::from_writer(io::stdout());

    for account in accounts.values() {
        let external_account: ExternalClientAccount = account.into();
        wtr.serialize(external_account)?;
    }

    wtr.flush()?;
    Ok(())
}
