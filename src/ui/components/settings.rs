use std::{io, path::PathBuf};

use actix::Addr;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error};

use crate::{
    actors::{fuel_calculator::FuelMessage, setup_manager::SetupChange},
    ui::components::{InputNumber, ThemeSwitcher},
    PROGRAM_NAME,
};

use super::theme::Theme;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub telemetry_laps: i32,
    pub reserve_laps: i32,
    pub theme: Theme,
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.save().unwrap()
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
    #[error("failed parsing settings file: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl Settings {
    pub fn path() -> Result<PathBuf, SettingsError> {
        #[cfg(windows)]
        let mut documents =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        #[cfg(not(windows))]
        let mut documents = PathBuf::from("./setups");
        documents.push(PROGRAM_NAME);
        if !documents.exists() {
            std::fs::create_dir_all(&documents)?
        }
        documents.push("settings.json");

        Ok(documents)
    }

    pub fn load() -> Result<Settings, SettingsError> {
        let data = std::fs::read_to_string(Settings::path()?)?;
        let settings: Settings = serde_json::from_str(&data)?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<(), SettingsError> {
        debug!("saving settings: {:?}", self);
        let file = std::fs::File::create(Settings::path()?)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn init() -> Settings {
        let settings = match Settings::load() {
            Ok(settings) => settings,
            Err(e) => {
                error!("failed to load settings: {e}");
                Settings::default()
            }
        };

        settings
    }
}

#[component]
pub fn SettingsComponent() -> Element {
    let mut settings: Signal<Settings> = use_context();
    // let setup_manager_tx = use_coroutine_handle::<SetupChange>();

    let telemetry_laps = use_signal(|| settings.read().telemetry_laps);
    let reserve_laps = use_signal(|| settings.read().reserve_laps);
    use_effect(move || {
        let router: Addr<crate::actors::Router> = use_context();
        debug!("changed laps: {:?}", telemetry_laps);
        router.do_send(SetupChange::TelemetryLaps(telemetry_laps()));
        settings.write().telemetry_laps = telemetry_laps();
    });
    use_effect(move || {
        let router: Addr<crate::actors::Router> = use_context();
        debug!("changed reserve laps: {:?}", reserve_laps);
        router.do_send(FuelMessage::ReserveLaps(reserve_laps()));
        settings.write().reserve_laps = reserve_laps();
    });

    static VERSION: &str = env!("CARGO_PKG_VERSION");

    rsx! {
        div { class: "grid grid-rows-[min-content,min-content,1fr,min-content] bg-base rounded-md shadow-lg p-4 gap-4",
            div { class: "grid gap-2",
                h1 { class: "text-xl", "App" }
                div { class: "label bg-surface0 rounded-md h-min px-2 pr-4",
                    span { class: "text-lg pl-8 label-text text-nowrap", "Theme" }
                    ThemeSwitcher { }
                }
            }
            div { class: "grid gap-2",
                h1 { class: "text-xl", "Setups" }
                InputNumber::<i32> {
                    name: "Telemetry Laps",
                    value: telemetry_laps,
                    min: 0,
                    max: 99,
                    step: 1
                }
                InputNumber::<i32> {
                    name: "Fuel Reserve Laps",
                    value: reserve_laps,
                    min: 0,
                    max: 99,
                    step: 1
                }
            }
            div { class: "grid gap-2 self-center" }
            div { class: "grid gap-2 justify-self-center",
                "Vapor Manager {VERSION}"
            }
        }
    }
}
