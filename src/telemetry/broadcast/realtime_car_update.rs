use nom::{
    number::complete::{le_f32, le_i16, le_i32, le_u16, u8},
    sequence::tuple,
};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::telemetry::broadcast::read_lap;

use super::{BroadcastNetworkProtocolInbound, CarLocation, InboundMessageTypes, LapInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealtimeCarUpdate {
    /// ID
    pub car_index: i16,
    /// Current Driver
    pub driver_index: i16,
    /// Total amount of drivers on car
    pub driver_count: u8,
    /// Gear
    /// R = 1
    /// N = 2
    /// 1 = 3
    /// ...
    pub gear: u8,
    pub world_pos_x: f32,
    pub world_pos_y: f32,
    pub yaw: f32,
    /// If car is in pitlane or track
    pub car_location: CarLocation,
    /// Speed
    pub kmh: u16,
    /// Official P/Q/R position total (1 based)
    pub position: u16,
    /// Official P/Q/R position within class (1 based)
    pub cup_position: u16,
    /// Position on track (1 based)
    pub track_position: u16,
    /// Track spline position between 0.0 and 1.0
    pub spline_position: f32,
    /// Completed Laps
    pub laps: u16,

    /// Realtime delta to best session lap
    pub delta: i32,
    pub best_session_lap: LapInfo,
    pub last_lap: LapInfo,
    pub current_lap: LapInfo,
}

impl BroadcastNetworkProtocolInbound for RealtimeCarUpdate {
    const TYPE: InboundMessageTypes = InboundMessageTypes::RealtimeCarUpdate;

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, (car_index, driver_index, driver_count)) = tuple((le_i16, le_i16, u8))(input)?;
        let (input, (gear, world_pos_x, world_pos_y, yaw, car_location, kmh)) =
            tuple((u8, le_f32, le_f32, le_f32, u8, le_u16))(input)?;
        let (input, (position, cup_position, track_position, spline_position, laps)) =
            tuple((le_u16, le_u16, le_u16, le_f32, le_u16))(input)?;

        let (input, (delta, best_session_lap, last_lap, current_lap)) =
            tuple((le_i32, read_lap, read_lap, read_lap))(input)?;

        Ok((
            input,
            Self {
                car_index,
                driver_index,
                driver_count,
                gear,
                world_pos_x,
                world_pos_y,
                yaw,
                car_location: CarLocation::from_primitive(car_location),
                kmh,
                position,
                cup_position,
                track_position,
                spline_position,
                laps,
                delta,
                best_session_lap,
                last_lap,
                current_lap,
            },
        ))
    }
}
