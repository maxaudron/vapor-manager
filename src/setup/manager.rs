use dioxus::prelude::*;
use futures_util::StreamExt;
use std::path::PathBuf;
use tracing::{debug, error};

use crate::{telemetry::Time, Weather, PROGRAM_NAME};

use super::{Setup, SetupError};

pub type Car = String;
pub type Track = String;
pub type FuelPerLap = f32;
pub type BestLap = Time;

pub enum SetupChange {
    Weather(Weather),
    SessionLength(f32),
    LapInfo((FuelPerLap, BestLap)),
    Load((Car, Track)),
}

#[derive(Debug, Default, Clone)]
pub struct SetupManager {
    pub track: Track,
    pub car: Car,
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
            setups: Vec::new(),
            adj_setups: Vec::new(),
            setup_folder,
            template_setup_folder,
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

        Ok(())
    }

    pub fn adjust_pressure(&mut self, air_temperature: u8, road_temperature: u8) {
        let mut new = self.setups.clone();
        new.iter_mut()
            .for_each(|setup| setup.adjust_pressure(air_temperature, road_temperature));
        self.adj_setups = new;
    }

    pub fn store(&mut self) {
        self.adj_setups
            .iter_mut()
            .for_each(|s| s.save(&self.setup_folder))
    }

    pub async fn coroutine(
        mut rx: UnboundedReceiver<SetupChange>,
        mut setup_manager: Signal<Option<SetupManager>>,
    ) {
        while let Some(msg) = rx.next().await {
            match msg {
                SetupChange::Weather(weather) => {
                    let mut manager = setup_manager.write();
                    if let Some(manager) = manager.as_mut() {
                        manager.adjust_pressure(weather.ambient_temp, weather.track_temp);
                        manager.store()
                    } else {
                        error!(
                            "got setup change weather message but setupmanager is not yet loaded"
                        );
                    }
                }
                SetupChange::Load((car, track)) => {
                    let mut manager = SetupManager::new(&track, &car);
                    match manager.discover() {
                        Ok(_) => (),
                        Err(e) => {
                            error!("failed discovering setups: {:?}", e)
                        }
                    }

                    *setup_manager.write() = Some(manager);
                }
                SetupChange::SessionLength(_) => todo!(),
                SetupChange::LapInfo(lapinfo) => {
                    debug!("updated lap info {lapinfo:?}")
                }
            }
        }
    }
}
