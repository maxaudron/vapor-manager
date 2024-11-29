#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(non_snake_case)]

use actors::ui::Weather;
use telemetry::{broadcast::LapType, shm::SessionType, LapTime, LapWheels};

mod actors;
// pub mod setup;
pub mod telemetry;

// mod widgets;
// pub mod components;

mod ui;

pub static PROGRAM_NAME: &'static str = "Vapor Manager";

#[derive(Debug, Clone, PartialEq, actix::Message)]
#[rtype(result = "()")]
pub enum StateChange {
    Weather(Weather),
    TrackName(String),
    SessionType(SessionType),
    ShmConnected(bool),
    BroadcastConnected(bool),
    LapWheels(LapWheels),
    LapTimeData(LapTimeData),
    Reset,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LapTimeData {
    pub number: i32,
    pub sectors: Vec<LapTime>,
    pub time: LapTime,
    pub valid: bool,
    pub lap_type: LapType,
}

use actix::prelude::*;

// #[actix::main]
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _system = System::new();

    ui::launch();
}
