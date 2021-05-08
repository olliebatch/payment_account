pub mod transactions;

mod accounts;
pub mod config;

use config::Config;
use std::error::Error;

type BoxError = Box<dyn Error + Send + Sync + 'static>;
