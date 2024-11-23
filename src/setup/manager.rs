use dioxus::prelude::*;
use futures_util::StreamExt;
use std::{path::PathBuf, time::Duration};
use tracing::{debug, error};

use crate::{
    // components::settings::Settings,
    telemetry::{broadcast::RaceSessionType, LapTime},
    Weather,
    PROGRAM_NAME,
};

use super::{Setup, SetupError, SetupMeta};

pub type Car = String;
pub type Track = String;
pub type FuelPerLap = f32;
pub type BestLap = LapTime;

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
pub enum SetupChange {
    Weather(Weather),
    SessionLength((RaceSessionType, Duration)),
    FuelPerLap(FuelPerLap),
    LapTime(BestLap),
    Load((Car, Track)),
    ReserveLaps(i32),
}

#[derive(Debug, Default, Clone)]
pub struct SetupManager {
    pub track: Track,
    pub car: Car,

    pub race_length: Duration,
    pub qualifying_length: Duration,
    pub fuel_per_lap: FuelPerLap,
    pub avg_lap: BestLap,
    pub race_fuel: i32,
    pub qualifying_fuel: i32,
    pub reserve_laps: i32,
    pub reserve_fuel_l: f32,

    /// Setup Templates loaded from disk
    pub setups: Vec<Setup>,
    /// Setups adjusted for track temperature to be saved
    pub adj_setups: Vec<Setup>,

    pub setup_folder: PathBuf,
    pub template_setup_folder: PathBuf,
}

impl Drop for SetupManager {
    fn drop(&mut self) {
        let meta = SetupMeta {
            avg_lap: self.avg_lap.clone(),
        };

        if self.template_setup_folder != PathBuf::default() {
            meta.save(&self.template_setup_folder);
        }
    }
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

        let meta = SetupMeta::read(&template_setup_folder);

        Self {
            track: track.to_owned(),
            car: car.to_owned(),
            setup_folder,
            template_setup_folder,
            avg_lap: meta.avg_lap,
            qualifying_length: Default::default(),
            race_length: Default::default(),
            fuel_per_lap: Default::default(),
            race_fuel: Default::default(),
            qualifying_fuel: Default::default(),
            reserve_laps: Default::default(),
            reserve_fuel_l: Default::default(),
            setups: Default::default(),
            adj_setups: Default::default(),
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
        self.save_fuel();
    }

    pub fn calculate_fuel(&mut self) {
        let best_millis = self.avg_lap.duration().as_millis();
        if !self.race_length.is_zero() && best_millis != 0 && self.fuel_per_lap != 0.0 {
            let laps = self.race_length.as_millis() / best_millis as u128;
            debug!(
                "calculating fuel: {:?} time {:?} l {:?} laps, reserve laps: {:?}",
                self.race_length, best_millis, laps, self.reserve_laps
            );
            let fuel = (((laps + self.reserve_laps as u128) as f32 * self.fuel_per_lap) * 1.1)
                .round() as i32;
            self.race_fuel = fuel;
        }

        if !self.qualifying_length.is_zero() && best_millis != 0 && self.fuel_per_lap != 0.0 {
            let laps = self.qualifying_length.as_millis() / best_millis as u128;
            debug!(
                "calculating fuel: {:?} time {:?} l {:?} laps, reserve laps: {:?}",
                self.qualifying_length, best_millis, laps, self.reserve_laps
            );
            let fuel = (((laps + self.reserve_laps as u128) as f32 * self.fuel_per_lap) * 1.1)
                .round() as i32;
            self.qualifying_fuel = fuel;
        }

        self.calculate_reserve_fuel();
        self.save_fuel();
    }

    pub fn save_fuel(&mut self) {
        self.adj_setups
            .iter_mut()
            .for_each(|setup| match setup.setup_type {
                super::SetupType::Qualifying => {
                    if self.qualifying_fuel > 0 {
                        setup.basic_setup.strategy.fuel = self.qualifying_fuel
                    }
                }
                _ => {
                    if self.race_fuel > 0 {
                        setup.basic_setup.strategy.fuel = self.race_fuel
                    }
                }
            })
    }

    pub fn calculate_reserve_fuel(&mut self) {
        self.reserve_fuel_l = self.reserve_laps as f32 * self.fuel_per_lap
    }

    pub fn store(&mut self) {
        self.adj_setups
            .iter_mut()
            .for_each(|s| s.save(&self.setup_folder))
    }

    // pub async fn coroutine(
    //     mut rx: UnboundedReceiver<SetupChange>,
    //     mut setup_manager: Signal<SetupManager>,
    //     settings: Signal<Settings>,
    // ) {
    //     while let Some(msg) = rx.next().await {
    //         match msg {
    //             SetupChange::Weather(weather) => {
    //                 debug!("got weather {weather:?}");
    //                 let mut manager = setup_manager.write();
    //                 manager.adjust_pressure(weather.ambient_temp, weather.track_temp);
    //                 manager.store()
    //             }
    //             SetupChange::Load((car, track)) => {
    //                 let mut manager = SetupManager::new(&track, &car);
    //                 match manager.discover() {
    //                     Ok(_) => (),
    //                     Err(e) => {
    //                         error!("failed discovering setups: {:?}", e)
    //                     }
    //                 }

    //                 debug!(
    //                     "got initial reserve laps: {:?}",
    //                     settings.read().reserve_laps
    //                 );
    //                 manager.reserve_laps = settings.read().reserve_laps;

    //                 setup_manager.set(manager);
    //             }
    //             SetupChange::SessionLength((session_type, duration)) => {
    //                 debug!("got session length {duration:?}");
    //                 let mut manager = setup_manager.write();
    //                 match session_type {
    //                     RaceSessionType::Qualifying => {
    //                         manager.qualifying_length = duration;
    //                         manager.calculate_fuel();
    //                     }
    //                     RaceSessionType::Race => {
    //                         manager.race_length = duration;
    //                         manager.calculate_fuel();
    //                     }
    //                     _ => (),
    //                 }
    //             }
    //             SetupChange::FuelPerLap(fuel_per_lap) => {
    //                 debug!("got fuel_per_lap {fuel_per_lap}");
    //                 let mut manager = setup_manager.write();
    //                 manager.fuel_per_lap = fuel_per_lap;
    //                 manager.calculate_fuel();
    //             }
    //             SetupChange::LapTime(lap_time) => {
    //                 debug!("got average lap time: {lap_time:?}");
    //                 let mut manager = setup_manager.write();
    //                 manager.avg_lap = lap_time;
    //                 manager.calculate_fuel();
    //             }
    //             SetupChange::ReserveLaps(laps) => {
    //                 debug!("got reserve laps: {laps}");
    //                 let mut manager = setup_manager.write();
    //                 manager.reserve_laps = laps;
    //                 manager.calculate_fuel();
    //             }
    //         }
    //     }
    // }
}
