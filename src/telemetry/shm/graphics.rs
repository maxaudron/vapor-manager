use super::SharedMemoryPage;

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct PageFileGraphics {
    /// Current step index
    pub packet_id: i32,
    /// See enums ACC_STATUS
    pub status: StatusRaw,
    /// See enums ACC_SESSION_TYPE
    pub session: SessionTypeRaw,
    /// Current lap time in wide character
    pub current_time: [u16; 15],
    /// Last lap time in wide character
    pub last_time: [u16; 15],
    /// Best lap time in wide character
    pub best_time: [u16; 15],
    /// Last split time in wide character
    pub split: [u16; 15],
    /// No of completed laps
    pub completed_laps: i32,
    /// Current player position
    pub position: i32,
    /// Current lap time in milliseconds
    pub i_current_time: i32,
    /// Last lap time in milliseconds
    pub i_last_time: i32,
    /// Best lap time in milliseconds
    pub i_best_time: i32,
    /// Session time left
    pub session_time_left: f32,
    /// Distance travelled in the current stint
    pub distance_traveled: f32,
    /// Car is pitting
    pub is_in_pit: i32,
    /// Current track sector
    pub current_sector_index: i32,
    /// Last sector time in milliseconds
    pub last_sector_time: i32,
    /// Number of completed laps
    pub number_of_laps: i32,
    /// Tyre compound used
    pub tyre_compound: [u16; 33],
    /// Not used in ACC
    pub replay_time_multiplier: f32,
    /// Car position on track spline (0.0 start to 1.0 finish)
    pub normalized_car_position: f32,
    /// Number of cars on track
    pub active_cars: i32,
    /// Coordinates of cars on track
    pub car_coordinates: [[f32; 3]; 60],
    /// Car IDs of cars on track
    pub car_id: [i32; 60],
    /// Player Car ID
    pub player_car_id: i32,
    /// Penalty time to wait
    pub penalty_time: f32,
    /// See enums ACC_FLAG_TYPE
    pub flag: FlagTypeRaw,
    /// See enums ACC_PENALTY_TYPE
    pub penalty: PenaltyRaw,
    /// Ideal line on
    pub ideal_line_on: i32,
    /// Car is in pit lane
    pub is_in_pit_lane: i32,
    /// Ideal line friction coefficient
    pub surface_grip: f32,
    /// Mandatory pit is completed
    pub mandatory_pit_done: i32,
    /// Wind speed in m/s
    pub wind_speed: f32,
    /// wind direction in radians
    pub wind_direction: f32,
    /// Car is working on setup
    pub is_setup_menu_visible: i32,
    /// current car main display index, see Appendix 1
    pub main_display_index: i32,
    /// current car secondary display index
    pub secondary_display_index: i32,
    /// Traction control level
    pub tc: i32,
    /// Traction control cut level
    pub tc_cut: i32,
    /// Current engine map
    pub engine_map: i32,
    /// ABS level
    pub abs: i32,
    /// Average fuel consumed per lap in liters
    pub fuel_used_per_lap: f32,
    /// Rain lights on
    pub rain_lights: i32,
    /// Flashing lights on
    pub flashing_lights: i32,
    /// Current lights stage
    pub lights_stage: i32,
    /// Exhaust temperature
    pub exhaust_temperature: f32,
    /// Current wiper stage
    pub wiper_lv: i32,
    /// Time the driver is allowed to drive/race (ms)
    pub driver_stint_total_time_left: i32,
    /// Time the driver is allowed to drive/stint (ms)
    pub driver_stint_time_left: i32,
    /// Are rain tyres equipped
    pub rain_tyres: i32,
    ///
    pub session_index: i32,
    /// Used fuel since last time refueling
    pub used_fuel: f32,
    /// Delta time in wide character
    pub delta_lap_time: [u16; 15],
    /// Delta time time in milliseconds
    pub i_delta_lap_time: i32,
    /// Estimated lap time in milliseconds
    pub estimated_lap_time: [u16; 15],
    /// Estimated lap time in wide character
    pub i_estimated_lap_time: i32,
    /// Delta positive (1) or negative (0)
    pub is_delta_positive: i32,
    /// Last split time in milliseconds
    pub i_split: i32,
    /// Check if Lap is valid for timing
    pub is_valid_lap: i32,
    /// Laps possible with current fuel level
    pub fuel_estimated_laps: f32,
    /// Status of track
    pub track_status: [u16; 33],
    /// Mandatory pitstops the player still has to do
    pub missing_mandatory_pits: i32,
    /// Time of day in seconds
    pub clock: f32,
    /// Is Blinker left on
    pub direction_lights_left: i32,
    /// Is Blinker right on
    pub direction_lights_right: i32,
    /// Yellow Flag is out?
    pub global_yellow: i32,
    /// Yellow Flag in Sector 1 is out?
    pub global_yellow1: i32,
    /// Yellow Flag in Sector 2 is out?
    pub global_yellow2: i32,
    /// Yellow Flag in Sector 3 is out?
    pub global_yellow3: i32,
    /// White Flag is out?
    pub global_white: i32,
    /// Green Flag is out?
    pub global_green: i32,
    /// Checkered Flag is out?
    pub global_chequered: i32,
    /// Red Flag is out?
    pub global_red: i32,
    /// Number of tyre set on the MFD
    pub mfd_tyre_set: i32,
    /// How much fuel to add on the MFD
    pub mfd_fuel_to_add: f32,
    /// Tyre pressure left front on the MFD
    pub mfd_tyre_pressure_lf: f32,
    /// Tyre pressure right front on the MFD
    pub mfd_tyre_pressure_rf: f32,
    /// Tyre pressure left rear on the MFD
    pub mfd_tyre_pressure_lr: f32,
    /// Tyre pressure right rear on the MFD
    pub mfd_tyre_pressure_rr: f32,
    /// See enums ACC_TRACK_GRIP_STATUS
    pub track_grip_status: TrackGripStatusRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity: RainIntensityRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity_in_10m: RainIntensityRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity_in_30m: RainIntensityRaw,
    /// Tyre Set currently in use
    pub current_tyre_set: i32,
    /// Next Tyre set per strategy
    pub strategy_tyre_set: i32,
    /// Distance in ms to car in front
    pub gap_ahead: i32,
    /// Distance in ms to car behind
    pub gap_behind: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct PenaltyRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct StatusRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct SessionTypeRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct FlagTypeRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct TrackGripStatusRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct RainIntensityRaw {
    pub data: i32,
}

impl SharedMemoryPage for PageFileGraphics {
    const NAME: &'static [u8; 21] = b"Local\\acpmf_graphics\0";

    fn debug_data() -> &'static Self {
        &PageFileGraphics {
            packet_id: 4363,
            status: StatusRaw { data: 2 },
            session: SessionTypeRaw { data: 3 },
            current_time: [48, 58, 50, 48, 58, 49, 51, 51, 0, 0, 0, 0, 0, 0, 0],
            last_time: [51, 53, 55, 57, 49, 58, 50, 51, 58, 54, 52, 55, 0, 0, 0],
            best_time: [51, 53, 55, 57, 49, 58, 50, 51, 58, 54, 52, 55, 0, 0, 0],
            split: [48, 58, 50, 48, 58, 49, 51, 51, 0, 0, 0, 0, 0, 0, 0],
            completed_laps: 0,
            position: 1,
            i_current_time: 20133,
            i_last_time: 2147483647,
            i_best_time: 2147483647,
            session_time_left: -1.0,
            distance_traveled: 0.1500693,
            is_in_pit: 0,
            current_sector_index: 0,
            last_sector_time: 0,
            number_of_laps: 0,
            tyre_compound: [
                100, 114, 121, 95, 99, 111, 109, 112, 111, 117, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            replay_time_multiplier: 0.0,
            normalized_car_position: 0.90051186,
            active_cars: 1,
            car_coordinates: [
                [-25.476627, -45.76898, 200.86961],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ],
            car_id: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            player_car_id: 0,
            penalty_time: 0.0,
            flag: FlagTypeRaw { data: 0 },
            penalty: PenaltyRaw { data: 0 },
            ideal_line_on: 0,
            is_in_pit_lane: 0,
            surface_grip: 0.0,
            mandatory_pit_done: 0,
            wind_speed: 0.0,
            wind_direction: 0.0,
            is_setup_menu_visible: 0,
            main_display_index: 0,
            secondary_display_index: 0,
            tc: 6,
            tc_cut: 0,
            engine_map: 1,
            abs: 6,
            fuel_used_per_lap: 2.93,
            rain_lights: 0,
            flashing_lights: 0,
            lights_stage: 0,
            exhaust_temperature: 130.76022,
            wiper_lv: 0,
            driver_stint_total_time_left: -1000,
            driver_stint_time_left: -1000,
            rain_tyres: 0,
            session_index: 0,
            used_fuel: 0.0,
            delta_lap_time: [45, 58, 45, 45, 58, 45, 45, 45, 0, 0, 0, 0, 0, 0, 0],
            i_delta_lap_time: 0,
            estimated_lap_time: [51, 53, 55, 57, 49, 58, 50, 51, 58, 54, 52, 55, 0, 0, 0],
            i_estimated_lap_time: 2147483647,
            is_delta_positive: 0,
            i_split: 20133,
            is_valid_lap: 1,
            fuel_estimated_laps: 3.412969,
            track_status: [
                79, 80, 84, 73, 77, 85, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            missing_mandatory_pits: 255,
            clock: 39600.0,
            direction_lights_left: 0,
            direction_lights_right: 0,
            global_yellow: 0,
            global_yellow1: 0,
            global_yellow2: 0,
            global_yellow3: 0,
            global_white: 0,
            global_green: 1,
            global_chequered: 0,
            global_red: 0,
            mfd_tyre_set: 2,
            mfd_fuel_to_add: 0.0,
            mfd_tyre_pressure_lf: 27.650002,
            mfd_tyre_pressure_rf: 27.650002,
            mfd_tyre_pressure_lr: 27.650002,
            mfd_tyre_pressure_rr: 27.650002,
            track_grip_status: TrackGripStatusRaw { data: 2 },
            rain_intensity: RainIntensityRaw { data: 0 },
            rain_intensity_in_10m: RainIntensityRaw { data: 0 },
            rain_intensity_in_30m: RainIntensityRaw { data: 0 },
            current_tyre_set: 2,
            strategy_tyre_set: 0,
            gap_ahead: 0,
            gap_behind: 0,
        }
    }
}
