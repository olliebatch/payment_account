use crate::BoxError;

pub struct Config {
    pub transaction_file: String,
}

impl Config {
    pub fn new() -> Result<Self, BoxError> {
        let file_path = std::env::args().nth(1).expect("Unable to get args");
        Ok(Config {
            transaction_file: file_path,
        })
    }
}
