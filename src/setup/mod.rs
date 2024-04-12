use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

pub struct SetupManager {}
impl SetupManager {
    pub fn discover(track: &str) {
        #[cfg(windows)]
        let docs = known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        let docs = "./setups";
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Setup {
    #[serde(rename = "carName")]
    car_name: String,
    #[serde(rename = "basicSetup")]
    basic_setup: BasicSetup,
    #[serde(rename = "advancedSetup")]
    advanced_setup: AdvancedSetup,
    #[serde(rename = "trackBopType")]
    track_bop_type: i32,
}

impl Setup {
    pub fn load(path: impl AsRef<Path>) -> Setup {
        let data = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    }

    pub fn save(&self, path: impl AsRef<Path>) {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, self).unwrap()
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BasicSetup {
    tyres: Tyres,
    alignment: Alignment,
    electronics: Electronics,
    strategy: Strategy,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tyres {
    #[serde(rename = "tyreCompound")]
    tyre_compound: i32,
    #[serde(rename = "tyrePressure")]
    tyre_pressure: [i32; 4],
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Alignment {
    camber: [i32; 4],
    #[serde(rename = "staticCamber")]
    static_camber: [f32; 4],
    toe: [i32; 4],
    #[serde(rename = "toeOutLinear")]
    toe_out_linear: [f32; 4],
    #[serde(rename = "casterLF")]
    caster_lf: i32,
    #[serde(rename = "casterRF")]
    caster_rf: i32,
    #[serde(rename = "steerRatio")]
    steer_ratio: i32,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Electronics {
    #[serde(rename = "tC1")]
    tc1: i32,
    #[serde(rename = "tC2")]
    tc2: i32,
    abs: i32,
    #[serde(rename = "eCUMap")]
    ecu_map: i32,
    #[serde(rename = "fuelMix")]
    fuel_mix: i32,
    #[serde(rename = "telemetryLaps")]
    telemetry_laps: i32,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Strategy {
    fuel: i32,
    #[serde(rename = "nPitStops")]
    n_pit_stops: i32,
    #[serde(rename = "tyreSet")]
    tyre_set: i32,
    #[serde(rename = "frontBrakePadCompound")]
    front_brake_pad_compound: i32,
    #[serde(rename = "rearBrakePadCompound")]
    rear_brake_pad_compound: i32,
    #[serde(rename = "pitStrategy")]
    pit_strategy: Vec<PitStrategy>,
    #[serde(rename = "fuelPerLap")]
    fuel_per_lap: f32,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PitStrategy {
    #[serde(rename = "fuelToAdd")]
    fuel_to_add: i32,
    tyres: Tyres,
    #[serde(rename = "tyreSet")]
    tyre_set: i32,
    #[serde(rename = "frontBrakePadCompound")]
    front_brake_pad_compound: i32,
    #[serde(rename = "rearBrakePadCompound")]
    rear_brake_pad_compound: i32,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AdvancedSetup {
    #[serde(rename = "mechanicalBalance")]
    mechanical_balance: MechanicalBalance,
    dampers: Dampers,
    #[serde(rename = "aeroBalance")]
    aero_balance: AeroBalance,
    drivetrain: Drivetrain,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MechanicalBalance {
    #[serde(rename = "aRBFront")]
    arb_front: i32,
    #[serde(rename = "aRBRear")]
    arb_rear: i32,
    #[serde(rename = "wheelRate")]
    wheel_rate: [i32; 4],
    #[serde(rename = "bumpStopRateUp")]
    bump_stop_rate_up: [i32; 4],
    #[serde(rename = "bumpStopRateDn")]
    bump_stop_rate_dn: [i32; 4],
    #[serde(rename = "bumpStopWindow")]
    bump_stop_window: [i32; 4],
    #[serde(rename = "brakeTorque")]
    brake_torque: i32,
    #[serde(rename = "brakeBias")]
    brake_bias: i32,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Dampers {
    #[serde(rename = "bumpSlow")]
    bump_slow: [i32; 4],
    #[serde(rename = "bumpFast")]
    bump_fast: [i32; 4],
    #[serde(rename = "reboundSlow")]
    rebound_slow: [i32; 4],
    #[serde(rename = "reboundFast")]
    rebound_fast: [i32; 4],
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AeroBalance {
    #[serde(rename = "rideHeight")]
    ride_height: [i32; 4],
    #[serde(rename = "rodLength")]
    rod_length: [f32; 4],
    splitter: i32,
    #[serde(rename = "rearWing")]
    rear_wing: i32,
    #[serde(rename = "brakeDuct")]
    brake_duct: [i32; 2],
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Drivetrain {
    preload: i32,
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Corners {
    front_left: i32,
    front_right: i32,
    rear_left: i32,
    rear_right: i32,
}
