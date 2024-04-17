use dioxus::prelude::*;
use futures_util::StreamExt;
use std::{path::PathBuf, time::Duration};
use tracing::{debug, error};

use crate::{components::settings::Settings, telemetry::Time, Weather, PROGRAM_NAME};

use super::{Setup, SetupError};

pub type Car = String;
pub type Track = String;
pub type FuelPerLap = f32;
pub type BestLap = Time;

pub enum SetupChange {
    Weather(Weather),
    SessionLength(Duration),
    FuelPerLap(FuelPerLap),
    BestLap(BestLap),
    Load((Car, Track)),
    ReserveLaps(i32),
}

#[derive(Debug, Default, Clone)]
pub struct SetupManager {
    pub track: Track,
    pub car: Car,

    pub session_length: Duration,
    pub fuel_per_lap: FuelPerLap,
    pub best_lap: BestLap,
    pub fuel: i32,
    pub reserve_laps: i32,
    pub reserve_fuel_l: f32,

    /// Setup Templates loaded from disk
    pub setups: Vec<Setup>,
    /// Setups adjusted for track temperature to be saved
    pub adj_setups: Vec<Setup>,

    pub setup_folder: PathBuf,
    pub template_setup_folder: PathBuf,
}

impl SetupManager {
    pub fn new(track: &str, car: &str) -> Self {
        #[cfg(windows)]
        let documents =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        #[cfg(not(windows))]
        let documents = PathBuf::from("./setups");

        let mut setup_folder = documents.clone();
        setup_folder.push("Assetto Corsa Competizione");
        setup_folder.push("Setups");
        setup_folder.push(car);
        setup_folder.push(track);

        let mut template_setup_folder = documents;
        template_setup_folder.push(PROGRAM_NAME);
        template_setup_folder.push("SetupTemplates");
        template_setup_folder.push(car);
        template_setup_folder.push(track);

        std::fs::create_dir_all(&template_setup_folder).unwrap();

        Self {
            track: track.to_owned(),
            car: car.to_owned(),
            setup_folder,
            template_setup_folder,
            ..Default::default()
        }
    }

    pub fn discover(&mut self) -> Result<(), SetupError> {
        debug!("loading setups from: {:?}", self.template_setup_folder);
        let setups =
            std::fs::read_dir(&self.template_setup_folder).map_err(|_| SetupError::NoSetups)?;

        self.setups = setups
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| (!f.path().is_dir()) && f.path().extension().is_some_and(|x| x == "json"))
            .map(|f| Setup::load(&f.path()))
            .collect();

        if let Some(setup) = self.setups.first() {
            self.fuel_per_lap = setup.basic_setup.strategy.fuel_per_lap
        }

        Ok(())
    }

    pub fn adjust_pressure(&mut self, air_temperature: u8, road_temperature: u8) {
        let mut new = self.setups.clone();
        new.iter_mut()
            .for_each(|setup| setup.adjust_pressure(air_temperature, road_temperature));
        self.adj_setups = new;
    }

    pub fn calculate_fuel(&mut self) {
        if !self.session_length.is_zero() && self.best_lap.millis != 0 && self.fuel_per_lap != 0.0 {
            let laps = self.session_length.as_millis() / self.best_lap.millis as u128;
            debug!(
                "calculating fuel: {:?} time {:?} l {:?} laps, reserve laps: {:?}",
                self.session_length, self.best_lap.millis, laps, self.reserve_laps
            );
            let fuel = (((laps + self.reserve_laps as u128) as f32 * self.fuel_per_lap) * 1.1)
                .round() as i32;
            self.fuel = fuel;
        }
    }

    pub fn calculate_reserve_fuel(&mut self) {
        self.reserve_fuel_l = self.reserve_laps as f32 * self.fuel_per_lap
    }

    pub fn store(&mut self) {
        self.adj_setups
            .iter_mut()
            .for_each(|s| s.save(&self.setup_folder))
    }

    pub async fn coroutine(
        mut rx: UnboundedReceiver<SetupChange>,
        mut setup_manager: Signal<SetupManager>,
        settings: Signal<Settings>,
    ) {
        while let Some(msg) = rx.next().await {
            match msg {
                SetupChange::Weather(weather) => {
                    debug!("got weather {weather:?}");
                    let mut manager = setup_manager.write();
                    manager.adjust_pressure(weather.ambient_temp, weather.track_temp);
                    manager.store()
                }
                SetupChange::Load((car, track)) => {
                    let mut manager = SetupManager::new(&track, &car);
                    match manager.discover() {
                        Ok(_) => (),
                        Err(e) => {
                            error!("failed discovering setups: {:?}", e)
                        }
                    }

                    debug!(
                        "got initial reserve laps: {:?}",
                        settings.read().reserve_laps
                    );
                    manager.reserve_laps = settings.read().reserve_laps;

                    setup_manager.set(manager);
                }
                SetupChange::SessionLength(duration) => {
                    debug!("got session length {duration:?}");
                    let mut manager = setup_manager.write();
                    manager.session_length = duration;
                    manager.calculate_fuel()
                }
                SetupChange::FuelPerLap(fuel_per_lap) => {
                    debug!("got fuel_per_lap {fuel_per_lap}");
                    let mut manager = setup_manager.write();
                    manager.fuel_per_lap = fuel_per_lap;
                    manager.calculate_fuel();
                    manager.calculate_reserve_fuel();
                }
                SetupChange::BestLap(best_lap) => {
                    debug!("got best_lap: {best_lap:?}");
                    let mut manager = setup_manager.write();
                    manager.best_lap = best_lap;
                    manager.calculate_fuel();
                    manager.calculate_reserve_fuel();
                }
                SetupChange::ReserveLaps(laps) => {
                    debug!("got reserve laps: {laps}");
                    let mut manager = setup_manager.write();
                    manager.reserve_laps = laps;
                    manager.calculate_fuel();
                    manager.calculate_reserve_fuel();
                }
            }
        }
    }
}
