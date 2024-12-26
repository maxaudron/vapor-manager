use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Setup {
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

#[test]
fn test_setup_serialize() {
    let setup = serde_json::to_string(&self::setup()).unwrap();
    let setup: Setup = serde_json::from_str(&setup).unwrap();
    assert_eq!(self::setup(), setup)
}

#[cfg(test)]
fn setup() -> Setup {
    Setup {
        car_name: "mercedes_amg_gt3_evo".to_string(),
        basic_setup: BasicSetup {
            tyres: Tyres {
                tyre_compound: 0,
                tyre_pressure: [53, 52, 56, 55],
            },
            alignment: Alignment {
                camber: [0, 0, 0, 0],
                static_camber: [-3.9203207, -3.920992, -4.2078676, -4.2085314],
                toe: [22, 22, 46, 46],
                toe_out_linear: [0.0004062106, 0.00040621002, 0.000184785, 0.0001847778],
                caster_lf: 44,
                caster_rf: 44,
                steer_ratio: 3,
            },
            electronics: Electronics {
                tc1: 5,
                tc2: 0,
                abs: 2,
                ecu_map: 0,
                fuel_mix: 0,
                telemetry_laps: 5,
            },
            strategy: Strategy {
                fuel: 0,
                n_pit_stops: 0,
                tyre_set: 27,
                front_brake_pad_compound: 0,
                rear_brake_pad_compound: 0,
                pit_strategy: vec![PitStrategy {
                    fuel_to_add: 0,
                    tyres: Tyres {
                        tyre_compound: 0,
                        tyre_pressure: [53, 52, 56, 55],
                    },
                    tyre_set: 2,
                    front_brake_pad_compound: 1,
                    rear_brake_pad_compound: 1,
                }],
                fuel_per_lap: 2.93,
            },
        },
        advanced_setup: AdvancedSetup {
            mechanical_balance: MechanicalBalance {
                arb_front: 10,
                arb_rear: 8,
                wheel_rate: [0, 0, 5, 5],
                bump_stop_rate_up: [0, 0, 0, 0],
                bump_stop_rate_dn: [0, 0, 10, 10],
                bump_stop_window: [5, 5, 9, 9],
                brake_torque: 20,
                brake_bias: 32,
            },
            dampers: Dampers {
                bump_slow: [5, 5, 5, 5],
                bump_fast: [6, 6, 6, 6],
                rebound_slow: [6, 6, 4, 4],
                rebound_fast: [5, 5, 8, 8],
            },
            aero_balance: AeroBalance {
                ride_height: [0, 6, 12, 18],
                rod_length: [-2.0982912, -2.0982912, 64.64535, 64.64535],
                splitter: 5,
                rear_wing: 9,
                brake_duct: [3, 3],
            },
            drivetrain: Drivetrain { preload: 10 },
        },
        track_bop_type: 44,
    }
}
