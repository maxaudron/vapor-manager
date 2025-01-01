use std::{collections::HashMap, time::Duration};

use actix::prelude::*;
use dioxus::signals::{SyncSignal, Writable};
use tracing::debug;

use crate::telemetry::{broadcast::LapType, LapTime, LapWheels};

use super::{fuel_calculator::FuelData, setup_manager::SetupFile, Reset, Router};

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct UiState {
    #[allow(unused)]
    router: Addr<Router>,
    session_info: SyncSignal<SessionInfo>,
    laps: SyncSignal<Laps>,
    setups: SyncSignal<Setups>,
    fuel_data: SyncSignal<FuelData>,
}

impl Actor for UiState {
    type Context = Context<Self>;
}

impl UiState {
    pub fn initialize(
        router: Addr<Router>,
        session_info: SyncSignal<SessionInfo>,
        laps: SyncSignal<Laps>,
        setups: SyncSignal<Setups>,
        fuel_data: SyncSignal<FuelData>,
    ) -> Addr<UiState> {
        let arb = Arbiter::new();
        let ui = UiState {
            router,
            session_info,
            laps,
            setups,
            fuel_data,
        };
        UiState::start_in_arbiter(&arb.handle(), |_| ui)
    }
}

#[derive(Debug, Default, Clone, Message)]
#[rtype(result = "()")]
pub struct SessionInfo {
    pub name: String,
    pub time: Duration,
    pub weather: Weather,
    pub live: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weather {
    pub ambient_temp: u8,
    pub track_temp: u8,
    pub clouds: u8,
    pub rain_level: u8,
    pub wetness: u8,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum UiUpdate {
    TrackName(String),
    Weather(Weather),
    SessionTime(Duration),
    SessionLive(bool),
    LapTime(LapTimeData),
    LapWheels(LapWheels),
    SetupTemplates(HashMap<String, SetupFile>),
    SetupAdjusted(HashMap<String, SetupFile>),
    FuelData(FuelData),
}

impl Handler<UiUpdate> for UiState {
    type Result = ();

    fn handle(&mut self, msg: UiUpdate, _ctx: &mut Self::Context) -> Self::Result {
        debug!(msg = ?msg);
        match msg {
            UiUpdate::TrackName(name) => self.session_info.write().name = name,
            UiUpdate::Weather(weather) => self.session_info.write().weather = weather,
            UiUpdate::SessionTime(time) => self.session_info.write().time = time,
            UiUpdate::SessionLive(live) => self.session_info.write().live = live,
            UiUpdate::LapTime(time) => self.laps.write().times.push(time),
            UiUpdate::LapWheels(wheels) => self.laps.write().wheels.push(wheels),
            UiUpdate::SetupTemplates(setups) => self.setups.write().templates = setups,
            UiUpdate::SetupAdjusted(setups) => self.setups.write().adjusted = setups,
            UiUpdate::FuelData(fuel) => self.fuel_data.write().replace(fuel),
        }
    }
}

#[derive(Debug, Default, Clone, Message)]
#[rtype(result = "()")]
pub struct Laps {
    pub times: Vec<LapTimeData>,
    pub wheels: Vec<LapWheels>,
}

impl Laps {
    pub fn reset(&mut self) {
        self.times.clear();
        self.wheels.clear();
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LapTimeData {
    pub number: i32,
    pub sectors: Vec<LapTime>,
    pub time: LapTime,
    pub valid: bool,
    pub lap_type: LapType,
}

#[derive(Debug, Default, Clone, Message)]
#[rtype(result = "()")]
pub struct Setups {
    pub templates: HashMap<String, SetupFile>,
    pub adjusted: HashMap<String, SetupFile>,
}

impl Handler<Reset> for UiState {
    type Result = ();

    fn handle(&mut self, _msg: Reset, _ctx: &mut Self::Context) -> Self::Result {
        self.laps.write().reset();
    }
}
