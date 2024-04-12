use super::SharedMemoryPage;

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct PageFilePhysics {
    /// Current step index
    pub packet_id: i32,
    /// Gas pedal input value (from -0 to 1.0)
    pub gas: f32,
    /// Brake pedal input value (from -0 to 1.0)
    pub brake: f32,
    /// Amount of fuel remaining in kg
    pub fuel: f32,
    /// Current gear
    pub gear: i32,
    /// Engine revolutions per minute
    pub rpm: i32,
    /// Steering input value (from -1.0 to 1.0)
    pub steer_angle: f32,
    /// Car speed in km/h
    pub speed_kmh: f32,
    /// Car velocity vector in global coordinates
    pub velocity: [f32; 3],
    /// Car acceleration vector in global coordinates
    pub acc_g: [f32; 3],
    /// Tyre slip for each tyre [FL, FR, RL, RR]
    pub wheel_slip: [f32; 4],
    /// Wheel load for each tyre [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub wheel_load: [f32; 4],
    /// Tyre pressure [FL, FR, RL, RR]
    pub wheels_pressure: [f32; 4],
    /// Wheel angular speed in rad/s [FL, FR, RL, RR]
    pub wheel_angular_speed: [f32; 4],
    /// Tyre wear [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_wear: [f32; 4],
    /// Dirt accumulated on tyre surface [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_dirty_level: [f32; 4],
    /// Tyre rubber core temperature [FL, FR, RL, RR]
    pub tyre_core_temperature: [f32; 4],
    /// Wheels camber in radians [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub camber_rad: [f32; 4],
    /// Suspension travel [FL, FR, RL, RR]
    pub suspension_travel: [f32; 4],
    /// DRS on *(NOT SENT BY SIM)*
    pub drs: f32,
    /// TC in action
    pub tc: f32,
    /// Car yaw orientation
    pub heading: f32,
    /// Car pitch orientation
    pub pitch: f32,
    /// Car roll orientation
    pub roll: f32,
    /// Centre of gravity height *(NOT SENT BY SIM)*
    pub cg_height: f32,
    /// Car damage: front 0, rear 1, left 2, right 3, centre 4
    pub car_damage: [f32; 5],
    /// Number of tyres out of track *(NOT SENT BY SIM)*
    pub number_of_tyres_out: i32,
    /// Pit limiter is on
    pub pit_limiter_on: i32,
    /// ABS in action
    pub abs: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_charge: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_input: f32,
    /// Automatic transmission on
    pub auto_shifter_on: i32,
    /// Ride height: 0 front, 1 rear *(NOT SENT BY SIM)*
    pub ride_height: [f32; 2],
    /// Car turbo level
    pub turbo_boost: f32,
    /// Car ballast in kg / Not implemented *(NOT SENT BY SIM)*
    pub ballast: f32,
    /// Air density *(NOT SENT BY SIM)*
    pub air_density: f32,
    /// Air temperature
    pub air_temp: f32,
    /// Road temperature
    pub road_temp: f32,
    /// Car angular velocity vector in local coordinates
    pub local_angular_vel: [f32; 3],
    /// Force feedback signal
    pub final_ff: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub performance_meter: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub engine_brake: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_recovery_level: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_power_level: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_heat_charging: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_is_charging: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_current_kj: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub drs_available: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub drs_enabled: i32,
    /// Brake discs temperatures
    pub brake_temp: [f32; 4],
    /// Clutch pedal input value (from -0 to 1.0)
    pub clutch: f32,
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_i: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_m: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_o: [f32; 4],
    /// Car is controlled by the AI
    pub is_ai_controlled: i32,
    /// Tyre contact point global coordinates [FL, FR, RL, RR]
    pub tyre_contact_point: [[f32; 3]; 4],
    /// Tyre contact normal [FL, FR, RL, RR] [x,y,z]
    pub tyre_contact_normal: [[f32; 3]; 4],
    /// Tyre contact heading [FL, FR, RL, RR] [x,y,z]
    pub tyre_contact_heading: [[f32; 3]; 4],
    /// Front brake bias, see Appendix 4
    pub brake_bias: f32,
    /// Car velocity vector in local coordinates
    pub local_velocity: [f32; 3],
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub p2p_activations: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub p2p_status: i32,
    /// Maximum engine rpm *(NOT SENT BY SIM)*
    pub current_max_rpm: i32,
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub mz: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub fx: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub fy: [f32; 4],
    /// Tyre slip ratio [FL, FR, RL, RR] in radians
    pub slip_ratio: [f32; 4],
    /// Tyre slip angle [FL, FR, RL, RR]
    pub slip_angle: [f32; 4],
    /// TC in action *(NOT SENT BY SIM)*
    pub tc_in_action: i32,
    /// ABS in action *(NOT SENT BY SIM)*
    pub abs_in_action: i32,
    /// Suspensions damage levels [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub suspension_damage: [f32; 4],
    /// Tyres core temperatures [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_temp: [f32; 4],
    /// Water Temperature
    pub water_temp: f32,
    /// Brake pressure [FL, FR, RL, RR] see Appendix 2
    pub brake_pressure: [f32; 4],
    /// Brake pad compund front
    pub front_brake_compound: i32,
    /// Brake pad compund rear
    pub rear_brake_compound: i32,
    /// Brake pad wear [FL, FR, RL, RR]
    pub pad_life: [f32; 4],
    /// Brake disk wear [FL, FR, RL, RR]
    pub disc_life: [f32; 4],
    /// Ignition switch set to on?
    pub ignition_on: i32,
    /// Starter Switch set to on?
    pub starter_engine_on: i32,
    /// Engine running?
    pub is_engine_running: i32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub kerb_vibration: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub slip_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub g_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub abs_vibrations: f32,
}

impl SharedMemoryPage for PageFilePhysics {
    const NAME: &'static [u8; 21] = b"Local\\acpmf_physics\0\0";

    fn debug_data() -> &'static Self {
        &PageFilePhysics {
            packet_id: 8032,
            gas: 0.0,
            brake: 0.0,
            fuel: 10.0,
            gear: 1,
            rpm: 0,
            steer_angle: 0.0,
            speed_kmh: 0.0037086266,
            velocity: [6.877405e-6, -4.0165825e-5, 0.0010258065],
            acc_g: [0.0, 0.0, 0.0],
            wheel_slip: [0.0036362559, 0.004135918, 0.0043424764, 0.004141719],
            wheel_load: [0.0, 0.0, 0.0, 0.0],
            wheels_pressure: [26.525536, 26.525337, 26.307251, 26.307045],
            wheel_angular_speed: [0.002992631, 0.002913558, 0.0058534076, 0.0058534076],
            tyre_wear: [0.0, 0.0, 0.0, 0.0],
            tyre_dirty_level: [0.0, 0.0, 0.0, 0.0],
            tyre_core_temperature: [80.737564, 80.735504, 80.5471, 80.54496],
            camber_rad: [0.0, 0.0, 0.0, 0.0],
            suspension_travel: [0.00019617245, 0.00020579257, 0.018344905, 0.01837896],
            drs: 0.0,
            tc: 0.0,
            heading: -0.054151792,
            pitch: -0.050406236,
            roll: -0.0027514559,
            cg_height: 0.0,
            car_damage: [0.0, 0.0, 0.0, 0.0, 0.0],
            number_of_tyres_out: 0,
            pit_limiter_on: 0,
            abs: 0.0,
            kers_charge: 0.0,
            kers_input: 0.0,
            auto_shifter_on: 0,
            ride_height: [0.0, 0.0],
            turbo_boost: 0.0,
            ballast: 0.0,
            air_density: 0.0,
            air_temp: 21.04263,
            road_temp: 26.245857,
            local_angular_vel: [8.316154e-7, -6.0104253e-6, -8.002335e-7],
            final_ff: -0.00015770944,
            performance_meter: 0.0,
            engine_brake: 0,
            ers_recovery_level: 0,
            ers_power_level: 0,
            ers_heat_charging: 0,
            ers_is_charging: 0,
            kers_current_kj: 0.0,
            drs_available: 0,
            drs_enabled: 0,
            brake_temp: [425.40823, 425.40823, 381.43036, 381.4226],
            clutch: 0.0,
            tyre_temp_i: [0.0, 0.0, 0.0, 0.0],
            tyre_temp_m: [0.0, 0.0, 0.0, 0.0],
            tyre_temp_o: [0.0, 0.0, 0.0, 0.0],
            is_ai_controlled: 0,
            tyre_contact_point: [
                [-24.522062, -46.21087, 202.12817],
                [-26.287497, -46.215683, 202.22363],
                [-24.684946, -46.111282, 199.54059],
                [-26.405115, -46.115925, 199.63358],
            ],
            tyre_contact_normal: [
                [-0.003867652, 0.9992327, 0.03897428],
                [-0.0023500451, 0.99923664, 0.038987458],
                [-0.005055331, 0.9991485, 0.040942594],
                [0.0011936651, 0.9992083, 0.039760925],
            ],
            tyre_contact_heading: [
                [-0.0520541, 0.038720563, -0.99789333],
                [-0.0559119, 0.03879531, -0.9976817],
                [-0.054512378, 0.04060686, -0.99768704],
                [-0.05383931, 0.039767466, -0.99775743],
            ],
            brake_bias: 0.746,
            local_velocity: [-4.8622933e-5, 1.1647165e-5, 0.0010253974],
            p2p_activations: 0,
            p2p_status: 0,
            current_max_rpm: 7900,
            mz: [0.0, 0.0, 0.0, 0.0],
            fx: [0.0, 0.0, 0.0, 0.0],
            fy: [0.0, 0.0, 0.0, 0.0],
            slip_ratio: [-1.6274702e-5, -1.5104464e-5, -0.00026961596, -0.00025955826],
            slip_angle: [0.00029594408, 0.00032866778, 0.00024531613, 0.00021854145],
            tc_in_action: 0,
            abs_in_action: 0,
            suspension_damage: [0.0, 0.0, 0.0, 0.0],
            tyre_temp: [80.737564, 80.735504, 80.5471, 80.54496],
            water_temp: 52.972134,
            brake_pressure: [0.0, 0.0, 0.0, 0.0],
            front_brake_compound: 1,
            rear_brake_compound: 1,
            pad_life: [29.000002, 29.000002, 29.000002, 29.000002],
            disc_life: [32.0, 32.0, 32.0, 32.0],
            ignition_on: 0,
            starter_engine_on: 0,
            is_engine_running: 0,
            kerb_vibration: 0.0,
            slip_vibrations: 1.296401e-6,
            g_vibrations: 0.0,
            abs_vibrations: 0.0,
        }
    }
}
