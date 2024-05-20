use super::*;

// #[test]
// fn test_setup_deserialize() {
//     let setup = Setup::load("./src/setup/21c_26c_aud_base.json");
//     assert_eq!(self::setup(), setup)
// }

#[test]
fn test_setup_serialize() {
    let setup = serde_json::to_string(&self::setup()).unwrap();
    let setup: Setup = serde_json::from_str(&setup).unwrap();
    assert_eq!(self::setup(), setup)
}

fn setup() -> Setup {
    Setup {
        name: "".to_string(),
        setup_type: SetupType::Base,
        air_temperature: 0,
        road_temperature: 0,
        path: "".into(),
        template_path: None,
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
