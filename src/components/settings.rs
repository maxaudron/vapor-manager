use std::{io, path::PathBuf};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error};

use crate::{
    components::{input::InputNumber, theme::ThemeSwitcher},
    PROGRAM_NAME,
};

use super::theme::Theme;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Settings {
    telemetry_laps: i32,
    theme: Theme,
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

    pub fn init(mut theme: Signal<Theme>) -> Settings {
        let settings = match Settings::load() {
            Ok(settings) => settings,
            Err(e) => {
                error!("failed to load settings: {e}");
                Settings::default()
            }
        };

        debug!("setting theme from settings: {:?}", theme);
        *theme.write() = settings.theme;

        settings
    }
}

#[component]
pub fn Settings() -> Element {
    let theme: Signal<Theme> = use_context();
    let mut settings: Signal<Settings> = use_context();

    let telemetry_laps = use_signal(|| settings.read().telemetry_laps);
    use_effect(move || {
        if settings.read().telemetry_laps != telemetry_laps() {
            debug!("changed laps: {:?}", telemetry_laps);
            settings.write().telemetry_laps = telemetry_laps();
        }

        if !theme.read().eq(&settings.read().theme) {
            debug!("use_effect setting theme: {:?}", theme);
            settings.write().theme = *theme.read()
        }
    });

    rsx! {
        div { class: "grid grid-rows-3 bg-base rounded-md shadow-lg p-4 mx-2 gap-4",
            div { class: "grid grid-rows-2",
                h1 { class: "text-lg", "App" }
                div { class: "label bg-surface0 rounded-md h-min px-2 pr-4",
                    span { class: "text-lg pl-8 label-text text-nowrap", "Theme" }
                    ThemeSwitcher { theme }
                }
            }
            div { class: "grid grid-rows-2",
                h1 { class: "text-lg", "Setups" }
                InputNumber::<i32> {
                    name: "Telemetry Laps",
                    value: telemetry_laps,
                    min: 0,
                    max: 99,
                    step: 1
                }
            }
        }
    }
}
