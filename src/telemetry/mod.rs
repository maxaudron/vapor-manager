use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

pub mod broadcast;
pub mod shm;

use shm::{AvgMinMax, Wheels};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LapTime(Duration);

impl From<Duration> for LapTime {
    fn from(value: Duration) -> Self {
        LapTime(value)
    }
}

impl Display for LapTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let minutes = secs / 60;
        let secs = secs - (minutes * 60);
        let millis = self.0.subsec_millis();
        write!(f, "{:02}:{:02}.{:03}", minutes, secs, millis)
    }
}

impl LapTime {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LapWheels {
    pub number: i32,
    pub tyre_pressure: AvgMinMax<Wheels<f32>>,
    pub tyre_temperature: AvgMinMax<Wheels<f32>>,
    pub brake_temperature: AvgMinMax<Wheels<f32>>,
}

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[cfg(windows)]
    #[error("failed to connect to acc: {0}")]
    ConnectionFailed(windows::core::Error),
    #[error("acc offline")]
    Offline,
}
