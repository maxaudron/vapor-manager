use std::time::Duration;

use actix::prelude::*;
use tracing::debug;

use crate::telemetry::LapTime;

use super::{setup_manager::SetupChange, ui::UiUpdate, Reset, Router};

#[derive(Debug, Clone)]
pub struct FuelCalculator {
    router: Addr<Router>,
    data: FuelData,
}

impl FuelCalculator {
    pub fn new(router: Addr<Router>) -> Self {
        Self {
            router,
            data: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FuelData {
    pub race_length: Duration,
    pub quali_length: Duration,

    // TODO track avg lap times gathered from saved data and session seperatly in future?
    pub avg_lap_time: LapTime,
    pub fuel_per_lap: f32,

    pub race_fuel: i32,
    pub quali_fuel: i32,
    pub reserve_fuel: i32,
    pub reserve_laps: i32,
}

impl FuelData {
    pub fn replace(&mut self, new: Self) {
        *self = new;
    }

    pub fn reset(&mut self) {
        self.race_length = Default::default();
        self.quali_length = Default::default();

        self.race_fuel = Default::default();
        self.quali_fuel = Default::default();
    }

    pub fn calculate_reserve(&mut self) -> Option<i32> {
        if self.fuel_per_lap != 0.0 && self.reserve_laps != 0 {
            self.reserve_fuel = (self.reserve_laps as f32 * self.fuel_per_lap).ceil() as i32;
            Some(self.reserve_fuel)
        } else {
            None
        }
    }

    pub fn calculate_race(&mut self) -> Option<i32> {
        let best_millis = self.avg_lap_time.duration().as_millis();
        if !self.race_length.is_zero() && best_millis != 0 && self.fuel_per_lap != 0.0 {
            let laps = self.race_length.as_millis() / best_millis as u128;
            debug!(
                "calculating fuel: {:?} time {:?} l {:?} laps, reserve laps: {:?}",
                self.race_length, best_millis, laps, self.reserve_laps
            );
            let fuel = (((laps + self.reserve_laps as u128) as f32 * self.fuel_per_lap) * 1.1)
                .ceil() as i32;

            self.race_fuel = fuel;
            Some(fuel)
        } else {
            None
        }
    }

    pub fn calculate_quali(&mut self) -> Option<i32> {
        let best_millis = self.avg_lap_time.duration().as_millis();
        if !self.quali_length.is_zero() && best_millis != 0 && self.fuel_per_lap != 0.0 {
            let laps = self.quali_length.as_millis() / best_millis as u128;
            debug!(
                "calculating fuel: {:?} time {:?} l {:?} laps, reserve laps: {:?}",
                self.quali_length, best_millis, laps, self.reserve_laps
            );
            let fuel = (((laps + self.reserve_laps as u128) as f32 * self.fuel_per_lap) * 1.1)
                .ceil() as i32;

            self.quali_fuel = fuel;
            Some(fuel)
        } else {
            None
        }
    }
}

impl Actor for FuelCalculator {
    type Context = Context<Self>;
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum FuelMessage {
    RaceLength(Duration),
    QualiLength(Duration),
    AvgLapTime(LapTime),
    FuelPerLap(f32),
    // FIXME
    // currently as whatever client would be connected sends it's updates to whatever Router is active
    // this would change even if secondary clients change their settings, maybe good if intended maybe not?
    ReserveLaps(i32),
}

impl Handler<FuelMessage> for FuelCalculator {
    type Result = ();

    fn handle(&mut self, msg: FuelMessage, _ctx: &mut Self::Context) -> Self::Result {
        debug!("{msg:?}");
        match msg {
            FuelMessage::RaceLength(l) => {
                self.data.race_length = l;
                if let Some(l) = self.data.calculate_race() {
                    self.router.do_send(SetupChange::RaceFuel(l))
                }
            }
            FuelMessage::QualiLength(l) => {
                self.data.quali_length = l;
                if let Some(l) = self.data.calculate_quali() {
                    self.router.do_send(SetupChange::QualiFuel(l))
                }
            }
            FuelMessage::AvgLapTime(t) => {
                self.data.avg_lap_time = t;
                if let Some(l) = self.data.calculate_race() {
                    self.router.do_send(SetupChange::RaceFuel(l))
                }
                if let Some(l) = self.data.calculate_quali() {
                    self.router.do_send(SetupChange::QualiFuel(l))
                }
            }
            FuelMessage::FuelPerLap(f) => {
                self.data.fuel_per_lap = f;
                if let Some(_) = self.data.calculate_reserve() {}
                if let Some(l) = self.data.calculate_race() {
                    self.router.do_send(SetupChange::RaceFuel(l))
                }
                if let Some(l) = self.data.calculate_quali() {
                    self.router.do_send(SetupChange::QualiFuel(l))
                }
            }
            FuelMessage::ReserveLaps(l) => {
                self.data.reserve_laps = l;
                if let Some(_) = self.data.calculate_reserve() {}
                if let Some(l) = self.data.calculate_race() {
                    self.router.do_send(SetupChange::RaceFuel(l))
                }
                if let Some(l) = self.data.calculate_quali() {
                    self.router.do_send(SetupChange::QualiFuel(l))
                }
            }
        }

        debug!("sending client update: {:?}", self.data);
        self.router.do_send(UiUpdate::FuelData(self.data.clone()));
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "Option<i32>")]
pub enum FuelRequest {
    RaceFuel,
    QualiFuel,
}

impl Handler<FuelRequest> for FuelCalculator {
    type Result = Option<i32>;

    fn handle(&mut self, msg: FuelRequest, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            FuelRequest::RaceFuel => self.data.calculate_race(),
            FuelRequest::QualiFuel => self.data.calculate_quali(),
        }
    }
}

impl Handler<Reset> for FuelCalculator {
    type Result = ();

    fn handle(&mut self, _msg: Reset, _ctx: &mut Self::Context) -> Self::Result {
        debug!("reset fuel calculator");
        self.data.reset();
    }
}
