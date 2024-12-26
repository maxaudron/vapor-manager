use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

use tracing::debug;

use crate::actors::ui::Weather;

use super::{Setup, SetupError};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct SetupFile {
    pub name: String,
    pub path: PathBuf,

    pub ambient_temperature: u8,
    pub track_temperature: u8,
    pub setup_type: SetupType,

    pub setup: Setup,
}

impl SetupFile {
    pub fn load(path: &Path) -> Result<Self, SetupError> {
        debug!("trying to load setup from {:?}", path);
        let data = std::fs::read_to_string(path)?;
        let setup: Setup = serde_json::from_str(&data)?;

        let (name, ambient_temperature, track_temperature) = Self::parse_name(path)?;
        let setup_type = SetupType::from_str(&name)?;
        let path = path.parent().unwrap().to_owned();

        Ok(SetupFile {
            ambient_temperature,
            track_temperature,
            setup_type,
            name,
            path,
            setup,
        })
    }

    pub fn parse_name(path: &Path) -> Result<(String, u8, u8), SetupError> {
        let file_name = path
            .file_name()
            .ok_or(SetupError::ParsePathError)?
            .to_str()
            .ok_or(SetupError::ParsePathError)?
            .strip_suffix(".json")
            .ok_or(SetupError::ParsePathError)?
            .to_owned();

        // TODO improve this to be more lax in parsing and guessing the air/track temperature
        // 21c 26c NAME
        // 012345678
        let ambient_temperature = (&file_name[0..2]).parse().unwrap();
        let track_temperature = (&file_name[4..6]).parse().unwrap();
        let name = file_name[8..].to_owned();

        Ok((name, ambient_temperature, track_temperature))
    }

    fn file_name(&self) -> PathBuf {
        PathBuf::from(format!(
            "{}c {}c {}.json",
            self.ambient_temperature, self.track_temperature, self.name
        ))
    }

    pub fn path_with_name(&self) -> PathBuf {
        self.path.join(self.file_name())
    }

    pub fn save(&mut self) {
        if !self.path.exists() {
            std::fs::create_dir_all(&self.path).unwrap();
        }

        let path = self.path_with_name();
        debug!("saving setup to {:?}", path);
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &self.setup).unwrap()
    }

    pub fn delete(&self) {
        let path = self.path_with_name();
        debug!("deleting setup: {:?}", path);
        std::fs::remove_file(path).unwrap();
    }

    pub fn adjust_weather(&mut self, weather: &Weather) {
        let diff = self.ambient_temperature as i8 - weather.ambient_temp as i8;
        debug!(
            "adjusting pressure to temp {:?} by {:?} clicks",
            weather.ambient_temp, diff
        );
        self.ambient_temperature = weather.ambient_temp;
        self.track_temperature = weather.track_temp;

        self.setup
            .basic_setup
            .tyres
            .tyre_pressure
            .iter_mut()
            .for_each(|i| *i += diff as i32);
        self.setup
            .basic_setup
            .strategy
            .pit_strategy
            .iter_mut()
            .for_each(|s| {
                s.tyres
                    .tyre_pressure
                    .iter_mut()
                    .for_each(|i| *i += diff as i32)
            });

        self.save();
    }
}

#[allow(unused)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SetupType {
    #[default]
    Base,
    Race,
    Qualifying,
}

impl std::str::FromStr for SetupType {
    type Err = SetupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.contains(" Q ") || s.contains(" Qual") {
            SetupType::Qualifying
        } else if s.contains(" R ") || s.contains(" Race ") {
            SetupType::Race
        } else {
            SetupType::Base
        })
    }
}

#[test]
#[cfg(test)]
fn test_parse_name() -> Result<(), SetupError> {
    let path = PathBuf::from("test\\21c 28c TEST Race Setup.json");

    let (name, ambient, track) = SetupFile::parse_name(&path)?;
    assert_eq!(name, "TEST Race Setup");
    assert_eq!(ambient, 21);
    assert_eq!(track, 28);

    Ok(())
}

#[test]
#[cfg(test)]
fn test_file_name() {
    let setup = SetupFile {
        name: "TEST Race Setup".to_owned(),
        path: PathBuf::from("test\\21c 28c TEST Race Setup.json"),
        ambient_temperature: 24,
        track_temperature: 31,
        setup_type: SetupType::Race,
        setup: Setup::default(),
    };

    let file_name = setup.file_name();
    assert_eq!(file_name, PathBuf::from("24c 31c TEST Race Setup.json"))
}
