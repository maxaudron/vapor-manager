use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

#[cfg(test)]
mod test;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("no setups found for track")]
    NoSetups,
}

pub struct SetupManager {
    pub track: String,
    pub car: String,
    pub setups: Vec<Setup>,
    pub adj_setups: Vec<Setup>,
}

impl SetupManager {
    pub fn discover(car: &str, track: &str) -> Result<SetupManager, SetupError> {
        #[cfg(windows)]
        let mut setup_dir =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        #[cfg(not(windows))]
        let docs = "./setups";

        setup_dir.push("Assetto Corsa Competizione");
        setup_dir.push("SetupTemplates");
        setup_dir.push(car);
        setup_dir.push(track);

        debug!("loading setups from: {:?}", setup_dir);
        let setups = std::fs::read_dir(setup_dir).map_err(|e| SetupError::NoSetups)?;

        Ok(SetupManager {
            track: track.to_string(),
            car: car.to_string(),
            setups: setups
                .into_iter()
                .filter_map(|f| f.ok())
                .filter(|f| {
                    (!f.path().is_dir()) && f.path().extension().is_some_and(|x| x == "json")
                })
                .map(|f| Setup::load(&f.path()))
                .collect(),
            adj_setups: Vec::new(),
        })
    }

    pub fn adjust_pressure(&mut self, air_temperature: i32, road_temperature: i32) {
        let mut new = self.setups.clone();
        new.iter_mut()
            .for_each(|setup| setup.adjust_pressure(air_temperature, road_temperature));
        self.adj_setups = new;
    }

    pub fn store(&self) {
        #[cfg(windows)]
        let mut setup_dir =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        #[cfg(not(windows))]
        let docs = "./setups";

        setup_dir.push("Assetto Corsa Competizione");
        setup_dir.push("Setups");
        setup_dir.push(&self.car);
        setup_dir.push(&self.track);

        self.adj_setups.iter().for_each(|s| s.save(&setup_dir))
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Setup {
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub air_temperature: i32,
    #[serde(skip)]
    pub road_temperature: i32,
    #[serde(rename = "carName")]
    pub car_name: String,
    #[serde(rename = "basicSetup")]
    pub basic_setup: BasicSetup,
    #[serde(rename = "advancedSetup")]
    pub advanced_setup: AdvancedSetup,
    #[serde(rename = "trackBopType")]
    pub track_bop_type: i32,
}

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
        // 0123456
        setup.air_temperature = (&name[0..2]).parse().unwrap();
        setup.road_temperature = (&name[4..6]).parse().unwrap();
        setup.name = name;

        setup
    }

    pub fn save(&self, path: &Path) {
        let name = format!(
            "{:?}c {:?}c {}",
            self.air_temperature,
            self.road_temperature,
            &self.name[8..self.name.len()]
        );
        let mut path = path.to_owned();
        path.push(format!("{}.json", name));

        debug!("saving setup to {:?}", path);
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, self).unwrap()
    }

    pub fn adjust_pressure(&mut self, air_temperature: i32, road_temperature: i32) {
        let diff = self.air_temperature - air_temperature;
        debug!(
            "adjusting pressure to temp {:?} by {:?} clicks",
            air_temperature, -diff
        );
        self.air_temperature = air_temperature;
        self.road_temperature = road_temperature;
        self.basic_setup
            .tyres
            .tyre_pressure
            .iter_mut()
            .for_each(|i| *i += -diff);
        self.basic_setup
            .strategy
            .pit_strategy
            .iter_mut()
            .for_each(|s| s.tyres.tyre_pressure.iter_mut().for_each(|i| *i += -diff))
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BasicSetup {
    pub tyres: Tyres,
    pub alignment: Alignment,
    pub electronics: Electronics,
    pub strategy: Strategy,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tyres {
    #[serde(rename = "tyreCompound")]
    pub tyre_compound: i32,
    #[serde(rename = "tyrePressure")]
    pub tyre_pressure: [i32; 4],
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Alignment {
    pub camber: [i32; 4],
    #[serde(rename = "staticCamber")]
    pub static_camber: [f32; 4],
    pub toe: [i32; 4],
    #[serde(rename = "toeOutLinear")]
    pub toe_out_linear: [f32; 4],
    #[serde(rename = "casterLF")]
    pub caster_lf: i32,
    #[serde(rename = "casterRF")]
    pub caster_rf: i32,
    #[serde(rename = "steerRatio")]
    pub steer_ratio: i32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Electronics {
    #[serde(rename = "tC1")]
    pub tc1: i32,
    #[serde(rename = "tC2")]
    pub tc2: i32,
    pub abs: i32,
    #[serde(rename = "eCUMap")]
    pub ecu_map: i32,
    #[serde(rename = "fuelMix")]
    pub fuel_mix: i32,
    #[serde(rename = "telemetryLaps")]
    pub telemetry_laps: i32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Strategy {
    pub fuel: i32,
    #[serde(rename = "nPitStops")]
    pub n_pit_stops: i32,
    #[serde(rename = "tyreSet")]
    pub tyre_set: i32,
    #[serde(rename = "frontBrakePadCompound")]
    pub front_brake_pad_compound: i32,
    #[serde(rename = "rearBrakePadCompound")]
    pub rear_brake_pad_compound: i32,
    #[serde(rename = "pitStrategy")]
    pub pit_strategy: Vec<PitStrategy>,
    #[serde(rename = "fuelPerLap")]
    pub fuel_per_lap: f32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PitStrategy {
    #[serde(rename = "fuelToAdd")]
    pub fuel_to_add: i32,
    pub tyres: Tyres,
    #[serde(rename = "tyreSet")]
    pub tyre_set: i32,
    #[serde(rename = "frontBrakePadCompound")]
    pub front_brake_pad_compound: i32,
    #[serde(rename = "rearBrakePadCompound")]
    pub rear_brake_pad_compound: i32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AdvancedSetup {
    #[serde(rename = "mechanicalBalance")]
    pub mechanical_balance: MechanicalBalance,
    pub dampers: Dampers,
    #[serde(rename = "aeroBalance")]
    pub aero_balance: AeroBalance,
    pub drivetrain: Drivetrain,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MechanicalBalance {
    #[serde(rename = "aRBFront")]
    pub arb_front: i32,
    #[serde(rename = "aRBRear")]
    pub arb_rear: i32,
    #[serde(rename = "wheelRate")]
    pub wheel_rate: [i32; 4],
    #[serde(rename = "bumpStopRateUp")]
    pub bump_stop_rate_up: [i32; 4],
    #[serde(rename = "bumpStopRateDn")]
    pub bump_stop_rate_dn: [i32; 4],
    #[serde(rename = "bumpStopWindow")]
    pub bump_stop_window: [i32; 4],
    #[serde(rename = "brakeTorque")]
    pub brake_torque: i32,
    #[serde(rename = "brakeBias")]
    pub brake_bias: i32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Dampers {
    #[serde(rename = "bumpSlow")]
    pub bump_slow: [i32; 4],
    #[serde(rename = "bumpFast")]
    pub bump_fast: [i32; 4],
    #[serde(rename = "reboundSlow")]
    pub rebound_slow: [i32; 4],
    #[serde(rename = "reboundFast")]
    pub rebound_fast: [i32; 4],
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AeroBalance {
    #[serde(rename = "rideHeight")]
    pub ride_height: [i32; 4],
    #[serde(rename = "rodLength")]
    pub rod_length: [f32; 4],
    pub splitter: i32,
    #[serde(rename = "rearWing")]
    pub rear_wing: i32,
    #[serde(rename = "brakeDuct")]
    pub brake_duct: [i32; 2],
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Drivetrain {
    pub preload: i32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Corners {
    pub front_left: i32,
    pub front_right: i32,
    pub rear_left: i32,
    pub rear_right: i32,
}
