pub mod transactions;

pub mod accounts;
pub mod config;

use std::error::Error;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;
