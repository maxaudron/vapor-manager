use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

#[cfg(test)]
mod test;

mod data;
mod setup;
mod manager;

pub use data::*;
pub use setup::*;
pub use manager::*;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("no setups found for track")]
    NoSetups,
}
