use thiserror::Error;

#[cfg(test)]
mod test;

mod data;
mod manager;
mod setup;
mod meta;

pub use data::*;
pub use manager::*;
pub use meta::*;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("no setups found for track")]
    NoSetups,
}
