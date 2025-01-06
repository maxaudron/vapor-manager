use std::{collections::HashMap, path::PathBuf, time::Duration};

use actix::prelude::*;

mod meta;
mod setup;
mod setup_file;
use meta::SetupMeta;
pub use setup::*;
pub use setup_file::*;
use thiserror::Error;
use tracing::{debug, error};

use super::{
    fuel_calculator::FuelMessage,
    ui::{UiUpdate, Weather},
    Router,
};

type Car = String;
type Track = String;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct SetupManager {
    pub router: Addr<Router>,

    pub weather: Weather,
    pub race_fuel: i32,
    pub quali_fuel: i32,
    pub telemetry_laps: i32,

    pub templates: HashMap<String, SetupFile>,
    pub setups: HashMap<String, SetupFile>,

    pub setup_folder: PathBuf,
    pub template_folder: PathBuf,

    setup_scheduled: Option<SpawnHandle>,
}

impl SetupManager {
    pub fn new(router: Addr<Router>) -> SetupManager {
        SetupManager {
            router,

            weather: Default::default(),
            race_fuel: Default::default(),
            quali_fuel: Default::default(),
            telemetry_laps: Default::default(),

            templates: Default::default(),
            setups: Default::default(),

            setup_folder: Default::default(),
            template_folder: Default::default(),

            setup_scheduled: None,
        }
    }
}

impl Actor for SetupManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.setup_paths().unwrap();
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("setup manager stopped");
        self.cleanup_setups();
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum SetupChange {
    Load(Car, Track),
    Weather(Weather),
    RaceFuel(i32),
    QualiFuel(i32),
    TelemetryLaps(i32),
}

// FIXME consistency
// i think this will all have consistency problems as it relies on timing
// the load of the setups before the broadcast api sends over it's information
//
// need to make it more resilient.
//  wait on setups to be loaded form disk?
//  cache information in manager and apply on load from disk?
impl Handler<SetupChange> for SetupManager {
    type Result = ();

    fn handle(&mut self, msg: SetupChange, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SetupChange::Weather(weather) => {
                self.weather = weather;
                self.adjust_weather(weather);
            }
            SetupChange::Load(car, track) => {
                if let Err(err) = self.load(car, track) {
                    error!("failed to load setups: {err}")
                }

                return;
            }
            SetupChange::RaceFuel(fuel) => {
                self.race_fuel = fuel;

                // Adjusts base and specific setups fuel
                // TODO maybe add a setting for not adjusting base setups
                self.adjust_fuel(fuel, SetupType::Base);
                self.adjust_fuel(fuel, SetupType::Race)
            }
            SetupChange::QualiFuel(fuel) => {
                self.quali_fuel = fuel;

                self.adjust_fuel(fuel, SetupType::Base);
                self.adjust_fuel(fuel, SetupType::Qualifying);
            }
            SetupChange::TelemetryLaps(laps) => {
                self.telemetry_laps = laps;
                self.adjust_telemetry_laps(laps);
            }
        };

        ctx.notify(CommitChanges::Schedule);
    }
}

impl SetupManager {
    fn setup_paths(&mut self) -> Result<(), SetupError> {
        #[cfg(windows)]
        let documents = known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        #[cfg(not(windows))]
        let documents = PathBuf::from("./setups");

        self.setup_folder = documents.clone();
        self.setup_folder.push("Assetto Corsa Competizione");
        self.setup_folder.push("Setups");

        self.template_folder = documents;
        self.template_folder.push(crate::PROGRAM_NAME);
        self.template_folder.push("SetupTemplates");

        std::fs::create_dir_all(&self.template_folder)?;

        Ok(())
    }

    fn load(&mut self, car: Car, track: Track) -> Result<(), SetupError> {
        let template_folder = self.template_folder.join(&car).join(&track);
        std::fs::create_dir_all(&template_folder)?;

        self.cleanup_setups();

        let setups = std::fs::read_dir(&template_folder).map_err(|_| SetupError::NoSetups)?;

        self.templates = setups
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| (!f.path().is_dir()) && f.path().extension().is_some_and(|x| x == "json"))
            .map(|f| SetupFile::load(&f.path()))
            .map(|setup| setup.and_then(|setup| Ok((setup.name.clone(), setup))))
            .collect::<Result<HashMap<String, SetupFile>, SetupError>>()?;

        self.setups = self
            .templates
            .iter()
            .map(|setup| {
                let (name, setup) = setup;
                let mut setup = setup.clone();
                setup.path = self.setup_folder.join(&car).join(&track);

                (name.clone(), setup)
            })
            .collect();

        // Send fuel per lap of whatever setup we get to fuelcalculator
        // to at least have some value in it
        if let Some((_, setup)) = self.setups.iter().next() {
            self.router.do_send(FuelMessage::FuelPerLap(
                setup.setup.basic_setup.strategy.fuel_per_lap,
            ));
        }

        // Read the avg lap time from meta file and send to fuelcalculator
        // to have a starting value to work with
        let meta = SetupMeta::read(&template_folder);
        debug!("loaded meta: {meta:?}");
        self.router.do_send(FuelMessage::AvgLapTime(meta.avg_lap));

        Ok(())
    }

    fn adjust_weather(&mut self, weather: Weather) {
        self.cleanup_setups();
        self.setups
            .iter_mut()
            .for_each(|(_, setup)| setup.adjust_weather(&weather));

        self.router.do_send(UiUpdate::SetupAdjusted(self.setups.clone()));
    }

    fn adjust_fuel(&mut self, fuel: i32, setup_type: SetupType) {
        self.setups
            .iter_mut()
            .filter(|(_, setup)| setup.setup_type == setup_type)
            .for_each(|(_, setup)| setup.adjust_fuel(fuel))
    }

    fn adjust_telemetry_laps(&mut self, laps: i32) {
        self.setups
            .iter_mut()
            .for_each(|(_, setup)| setup.adjust_telemetry_laps(laps));
    }

    fn cleanup_setups(&self) {
        self.setups.iter().for_each(|(_name, setup)| {
            setup.delete().unwrap_or_else(|e| {
                error!("failed to delete setup: {e:?}");
                ()
            });
        })
    }
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("no setups found for track")]
    NoSetups,
    #[error("io error {0}")]
    IoError(#[from] std::io::Error),
    #[error("failed to parse setup path")]
    ParsePathError,
    #[error("failed to parse setup file")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
enum CommitChanges {
    Schedule,
    Handle,
}

impl Handler<CommitChanges> for SetupManager {
    type Result = ();

    fn handle(&mut self, msg: CommitChanges, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            CommitChanges::Schedule => {
                if let Some(handle) = self.setup_scheduled {
                    ctx.cancel_future(handle);
                }

                ctx.notify_later(CommitChanges::Handle, Duration::from_millis(500));
            }
            CommitChanges::Handle => {
                self.setups.iter_mut().for_each(|(_, s)| s.save());
                self.router.do_send(UiUpdate::SetupAdjusted(self.setups.clone()));
            }
        }
    }
}
