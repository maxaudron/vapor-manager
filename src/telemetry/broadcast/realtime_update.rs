use nom::{
    number::complete::{le_f32, le_i32, le_u16, u8},
    sequence::tuple,
};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::telemetry::broadcast::{read_lap, read_string};

use super::{
    write_string, BroadcastNetworkProtocolInbound, InboundMessageTypes, LapInfo, RaceSessionType,
    SessionPhase,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct RealtimeUpdate {
    pub event_index: u16,
    pub session_index: u16,
    pub session_type: RaceSessionType,
    pub phase: SessionPhase,
    pub session_time: f32,
    pub session_end_time: f32,

    pub focused_car_index: i32,
    pub active_camera_set: String,
    pub active_camera: String,
    pub current_hud_page: String,

    pub replay_playing: bool,
    pub replay_session_time: Option<f32>,
    pub replay_remaining_time: Option<f32>,

    pub time_of_day: f32,
    pub ambient_temp: u8,
    pub track_temp: u8,
    pub clouds: u8,
    pub rain_level: u8,
    pub wetness: u8,

    pub best_session_lap: LapInfo,
}

impl RealtimeUpdate {
    pub fn session_length(&self) -> f32 {
        self.session_end_time + self.session_time
    }
}

impl BroadcastNetworkProtocolInbound for RealtimeUpdate {
    const TYPE: InboundMessageTypes = InboundMessageTypes::RealtimeUpdate;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.push(Self::TYPE as u8);
        out.extend(self.event_index.to_le_bytes());
        out.extend(self.session_index.to_le_bytes());
        out.push(self.session_type.into());
        out.push(self.phase.into());
        out.extend(self.session_time.to_le_bytes());
        out.extend(self.session_end_time.to_le_bytes());

        out.extend(self.focused_car_index.to_le_bytes());
        out.extend(write_string(&self.active_camera_set));
        out.extend(write_string(&self.active_camera));
        out.extend(write_string(&self.current_hud_page));

        out.push(self.replay_playing as u8);
        if self.replay_playing {
            out.extend(self.replay_session_time.unwrap_or_default().to_le_bytes());
            out.extend(self.replay_remaining_time.unwrap_or_default().to_le_bytes());
        }

        out.extend(self.time_of_day.to_le_bytes());
        out.push(self.ambient_temp);
        out.push(self.track_temp);
        out.push(self.clouds);
        out.push(self.rain_level);
        out.push(self.wetness);

        // FIXME i'm lazy
        out.extend([
            255, 255, 255, 127, 0, 0, 0, 0, 3, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255,
            255, 127, 0, 1, 0, 0,
        ]);

        out
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (
            input,
            (event_index, session_index, session_type, phase, session_time, session_end_time),
        ) = tuple((le_u16, le_u16, u8, u8, le_f32, le_f32))(input)?;

        let (input, (focused_car_index, active_camera_set, active_camera, current_hud_page)) =
            tuple((le_i32, read_string, read_string, read_string))(input)?;

        let (mut input, replay_playing) = u8(input)?;
        let replay_playing = replay_playing > 0;
        let (mut replay_session_time, mut replay_remaining_time) = (None, None);
        if replay_playing {
            let (out, rs) = le_f32(input)?;
            let (out, rr) = le_f32(out)?;
            replay_session_time = Some(rs);
            replay_remaining_time = Some(rr);

            input = out;
        }

        let (input, (time_of_day, ambient_temp, track_temp, clouds, rain_level, wetness)) =
            tuple((le_f32, u8, u8, u8, u8, u8))(input)?;

        let (input, best_session_lap) = read_lap(input)?;

        Ok((
            input,
            Self {
                event_index,
                session_index,
                session_type: RaceSessionType::from_primitive(session_type),
                phase: SessionPhase::from_primitive(phase),
                session_time,
                session_end_time,
                focused_car_index,
                active_camera_set: active_camera_set.to_owned(),
                active_camera: active_camera.to_owned(),
                current_hud_page: current_hud_page.to_owned(),
                replay_playing,
                replay_session_time,
                replay_remaining_time,
                time_of_day,
                ambient_temp,
                track_temp,
                clouds,
                rain_level,
                wetness,
                best_session_lap,
            },
        ))
    }
}

#[test]
fn test_realtime_update_deserialize() {
    let buf = [
        0, 0, 0, 0, 0, 5, 0, 75, 197, 70, 106, 47, 90, 74, 0, 0, 0, 0, 8, 0, 68, 114, 105, 118, 97,
        98, 108, 101, 7, 0, 67, 111, 99, 107, 112, 105, 116, 9, 0, 66, 97, 115, 105, 99, 32, 72,
        85, 68, 0, 70, 249, 68, 71, 27, 37, 0, 0, 0, 255, 255, 255, 127, 0, 0, 0, 0, 3, 255, 255,
        255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 0, 1, 0, 0, 0, 0, 0, 0,
    ];

    let update = RealtimeUpdate {
        event_index: 0,                            // 2, 0
        session_index: 0,                          // 0, 0
        session_type: RaceSessionType::Practice,   // 0
        phase: SessionPhase::Session,              // 0
        session_time: 25253.5,                     // 5, 0, 75, 197,
        session_end_time: 3574746.5,               // 70, 106, 47, 90,
        focused_car_index: 0,                      // 74, 0, 0, 0,
        active_camera_set: "Drivable".to_string(), // 8, 0, 68, 114, 105, 118, 97, 98, 108, 101,
        active_camera: "Cockpit".to_string(),      // 7, 0, 67, 111, 99, 107, 112, 105, 116,
        current_hud_page: "Basic HUD".to_string(), // 9, 0, 66, 97, 115, 105, 99, 32, 72, 85, 68
        replay_playing: false,                     // 0
        replay_session_time: None,
        replay_remaining_time: None,
        time_of_day: 50425.273, // 70, 249, 68, 71
        ambient_temp: 27,       // 27
        track_temp: 37,         // 37
        clouds: 0,              // 0
        rain_level: 0,          // 0
        wetness: 0,             // 0
        best_session_lap: LapInfo {
            lap_type: crate::telemetry::broadcast::LapType::Regular,
            laptime: None,
            car_index: 0,
            driver_index: 0,
            splits: vec![None, None, None],
            invalid: false,
            valid_for_best: true,
        },
    };

    let (_input, parsed) =
        <RealtimeUpdate as BroadcastNetworkProtocolInbound>::deserialize(&buf[..]).unwrap();

    assert_eq!(update, parsed)
}
