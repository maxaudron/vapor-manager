use std::{
    fs::File,
    path::{Path, PathBuf},
};

use tracing::debug;

use crate::setup::{Setup, SetupType};

impl Setup {
    pub fn load(path: &Path) -> Setup {
        debug!("trying to load setup from {:?}", path);
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".json")
            .unwrap()
            .to_owned();
        let data = std::fs::read_to_string(path).unwrap();
        let mut setup: Setup = serde_json::from_str(&data).unwrap();

        // 21c 26c NAME
        // 012345678
        setup.air_temperature = (&name[0..2]).parse().unwrap();
        setup.road_temperature = (&name[4..6]).parse().unwrap();
        setup.path = path.to_owned();

        setup.setup_type = if name.contains(" Q ") {
            SetupType::Qualifying
        } else if name.contains(" R ") {
            SetupType::Race
        } else {
            SetupType::Base
        };
        
        setup.name = name;

        setup
    }

    pub fn path_with_name(&self, path: &Path) -> PathBuf {
        let mut path = path.to_owned();
        path.push(format!("{}.json", self.name));
        path
    }

    pub fn save(&mut self, path: &Path) {
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap();
        }

        let path = self.path_with_name(path);
        debug!("saving setup to {:?}", path);
        self.template_path = Some(path.clone());
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, self).unwrap()
    }

    pub fn delete(&self, path: &Path) {
        let path = self.path_with_name(path);
        debug!("deleting setup: {:?}", path);
        std::fs::remove_file(path).unwrap();
    }

    pub fn adjust_pressure(&mut self, air_temperature: u8, road_temperature: u8) {
        let diff = self.air_temperature as i8 - air_temperature as i8;
        debug!(
            "adjusting pressure to temp {:?} by {:?} clicks",
            air_temperature, diff
        );
        self.name = format!(
            "{:?}c {:?}c {}",
            air_temperature,
            road_temperature,
            &self.name[8..self.name.len()]
        );
        self.air_temperature = air_temperature;
        self.road_temperature = road_temperature;
        self.basic_setup
            .tyres
            .tyre_pressure
            .iter_mut()
            .for_each(|i| *i += diff as i32);
        self.basic_setup
            .strategy
            .pit_strategy
            .iter_mut()
            .for_each(|s| {
                s.tyres
                    .tyre_pressure
                    .iter_mut()
                    .for_each(|i| *i += diff as i32)
            })
    }

    pub fn set_fuel(&mut self, fuel: i32) {
        self.basic_setup.strategy.fuel = fuel
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        if let Some(path) = self.template_path.as_ref() {
            debug!("deleting setup {:?}", path);
            let _ = std::fs::remove_file(path);
        }
    }
}
