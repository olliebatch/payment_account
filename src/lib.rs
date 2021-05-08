pub mod transactions;

pub mod config;

use config::Config;
use std::error::Error;

type BoxError = Box<dyn Error + Send + Sync + 'static>;
