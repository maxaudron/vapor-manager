use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Setup {
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub air_temperature: u8,
    #[serde(skip)]
    pub road_temperature: u8,
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub template_path: Option<PathBuf>,
    #[serde(rename = "carName")]
    pub car_name: String,
    #[serde(rename = "basicSetup")]
    pub basic_setup: BasicSetup,
    #[serde(rename = "advancedSetup")]
    pub advanced_setup: AdvancedSetup,
    #[serde(rename = "trackBopType")]
    pub track_bop_type: i32,
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