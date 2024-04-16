use thiserror::Error;

#[cfg(test)]
mod test;

mod data;
mod setup;
mod manager;

pub use data::*;
pub use manager::*;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("no setups found for track")]
    NoSetups,
}
