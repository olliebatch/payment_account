use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use csv::StringRecord;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Row {
    payment_type: String,
    client: u16,
    tx: u32,
    population: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = std::env::args().nth(1).expect("no csv given");
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        println!("{:?}", row);
    }
    Ok(())
}
