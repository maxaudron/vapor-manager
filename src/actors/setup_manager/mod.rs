use std::{collections::HashMap, path::PathBuf};

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
    ui::{SessionInfo, UiUpdate, Weather},
    Router,
};

type Car = String;
type Track = String;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct SetupManager {
    pub router: Addr<Router>,

    pub weather: SessionInfo,

    pub templates: HashMap<String, SetupFile>,
    pub setups: HashMap<String, SetupFile>,

    pub setup_folder: PathBuf,
    pub template_folder: PathBuf,
}

impl SetupManager {
    pub fn new(router: Addr<Router>) -> SetupManager {
        SetupManager {
            router,
            weather: Default::default(),
            templates: Default::default(),
            setups: Default::default(),
            setup_folder: Default::default(),
            template_folder: Default::default(),
        }
    }
}

impl Actor for SetupManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.setup_paths().unwrap();
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum SetupChange {
    Load(Car, Track),
    Weather(Weather),
}

impl Handler<SetupChange> for SetupManager {
    type Result = ();

    fn handle(&mut self, msg: SetupChange, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SetupChange::Weather(weather) => self.adjust_weather(weather),
            SetupChange::Load(car, track) => {
                if let Err(err) = self.load(car, track) {
                    error!("failed to load setups: {err}")
                }
            }
            _ => (),
        };
    }
}

impl SetupManager {
    fn setup_paths(&mut self) -> Result<(), SetupError> {
        #[cfg(windows)]
        let documents =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
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

        self.router
            .do_send(UiUpdate::SetupTemplates(self.templates.clone()));

        let meta = SetupMeta::read(&template_folder);

        Ok(())
    }

    fn adjust_weather(&mut self, weather: Weather) {
        self.setups
            .iter_mut()
            .for_each(|(_, setup)| setup.adjust_weather(&weather));

        self.router
            .do_send(UiUpdate::SetupAdjusted(self.setups.clone()));
    }
}

impl Drop for SetupManager {
    fn drop(&mut self) {
        self.setups.iter().for_each(|(_name, setup)| setup.delete())
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
