use super::SharedMemoryPage;

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct PageFileStatic {
    /// Shared memory version
    pub sm_version: [u16; 15],
    /// Assetto Corsa version
    pub ac_version: [u16; 15],
    /// Number of sessions
    pub number_of_sessions: i32,
    /// Number of cars
    pub num_cars: i32,
    /// Player car model see Appendix 2
    pub car_model: [u16; 33],
    /// Track name
    pub track: [u16; 33],
    /// Player name
    pub player_name: [u16; 33],
    /// Player surname
    pub player_surname: [u16; 33],
    /// Player nickname
    pub player_nick: [u16; 33],
    /// Number of sectors
    pub sector_count: i32,
    /// Not shown in ACC
    pub max_torque: f32,
    /// Not shown in ACC
    pub max_power: f32,
    /// Maximum rpm
    pub max_rpm: i32,
    /// Maximum fuel tank capacity
    pub max_fuel: f32,
    /// Not shown in ACC
    pub suspension_max_travel: [f32; 4],
    /// Not shown in ACC
    pub tyre_radius: [f32; 4],
    /// Maximum turbo boost
    pub max_turbo_boost: f32,
    ///  
    pub deprecated_1: f32,
    ///  
    pub deprecated_2: f32,
    /// Penalties enabled
    pub penalties_enabled: i32,
    /// Fuel consumption rate
    pub aid_fuel_rate: f32,
    /// Tyre wear rate
    pub aid_tyre_rate: f32,
    /// Mechanical damage rate
    pub aid_mechanical_damage: f32,
    /// Not allowed in Blancpain endurance series
    pub aid_allow_tyre_blankets: f32,
    /// Stability control used
    pub aid_stability: f32,
    /// Auto clutch used
    pub aid_auto_clutch: i32,
    /// Always true in ACC
    pub aid_auto_blip: i32,
    /// Not used in ACC
    pub has_drs: i32,
    /// Not used in ACC
    pub has_ers: i32,
    /// Not used in ACC
    pub has_kers: i32,
    /// Not used in ACC
    pub kers_max_j: f32,
    /// Not used in ACC
    pub engine_brake_settings_count: i32,
    /// Not used in ACC
    pub ers_power_controller_count: i32,
    /// Not used in ACC
    pub track_spline_length: f32,
    /// Not used in ACC
    pub track_configuration: [u16; 33],
    /// Not used in ACC
    pub ers_max_j: f32,
    /// Not used in ACC
    pub is_timed_race: i32,
    /// Not used in ACC
    pub has_extra_lap: i32,
    /// Not used in ACC
    pub car_skin: [u16; 33],
    /// Not used in ACC
    pub reversed_grid_positions: i32,
    /// Pit window opening time
    pub pit_window_start: i32,
    /// Pit windows closing time
    pub pit_window_end: i32,
    /// If is a multiplayer session
    pub is_online: i32,
    /// Name of the dry tyres
    pub dry_tyres_name: [u16; 33],
    /// Name of the wet tyres
    pub wet_tyres_name: [u16; 33],
}

impl SharedMemoryPage for PageFileStatic {
    const NAME: &'static [u8; 21] = b"Local\\acpmf_static\0\0\0";

    fn debug_data() -> &'static Self {
        &PageFileStatic {
            sm_version: [49, 46, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ac_version: [49, 46, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            number_of_sessions: 0,
            num_cars: 1,
            car_model: [
                109, 101, 114, 99, 101, 100, 101, 115, 95, 97, 109, 103, 95, 103, 116, 51, 95, 101,
                118, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            track: [
                75, 121, 97, 108, 97, 109, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            player_name: [
                77, 97, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0,
            ],
            player_surname: [
                65, 117, 100, 114, 111, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            player_nick: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ],
            sector_count: 3,
            max_torque: 0.0,
            max_power: 0.0,
            max_rpm: 7900,
            max_fuel: 120.0,
            suspension_max_travel: [0.0, 0.0, 0.0, 0.0],
            tyre_radius: [0.0, 0.0, 0.0, 0.0],
            max_turbo_boost: 0.0,
            deprecated_1: 0.0,
            deprecated_2: 0.0,
            penalties_enabled: 1,
            aid_fuel_rate: 0.0,
            aid_tyre_rate: 0.0,
            aid_mechanical_damage: 0.0,
            aid_allow_tyre_blankets: 0.0,
            aid_stability: 0.0,
            aid_auto_clutch: 1,
            aid_auto_blip: 1,
            has_drs: 0,
            has_ers: 0,
            has_kers: 0,
            kers_max_j: 0.0,
            engine_brake_settings_count: 0,
            ers_power_controller_count: 0,
            track_spline_length: 0.0,
            track_configuration: [
                116, 114, 97, 99, 107, 32, 99, 111, 110, 102, 105, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            ers_max_j: 0.0,
            is_timed_race: 0,
            has_extra_lap: 0,
            car_skin: [
                115, 107, 105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            reversed_grid_positions: 0,
            pit_window_start: 0,
            pit_window_end: -1000,
            is_online: 0,
            dry_tyres_name: [
                68, 72, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ],
            wet_tyres_name: [
                87, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ],
        }
    }
}
