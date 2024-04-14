#![warn(clippy::all, rust_2018_idioms)]

use telemetry::{SessionType, Wheels};

pub mod setup;
pub mod telemetry;

// mod widgets;
pub mod components;

#[derive(Debug, Clone)]
pub enum StateChange {
    AvgTyrePressure(Wheels<f32>),
    Weather(Weather),
    TrackName(String),
    SessionType(SessionType),
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub avg_tyre_pressures: Wheels<f32>,
    pub weather: Weather,
    pub track_name: String,
    pub session_type: SessionType,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weather {
    pub ambient_temp: u8,
    pub track_temp: u8,
    pub clouds: u8,
    pub rain_level: u8,
    pub wetness: u8,
}

