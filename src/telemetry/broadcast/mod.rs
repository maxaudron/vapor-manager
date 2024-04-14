use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

use dioxus::hooks::UnboundedSender;
use tracing::{debug, error};
use nom::{
    bytes::complete::take,
    number::complete::{le_u16, u8},
    IResult,
};

mod broadcasting_event;
mod data;
mod entrylist;
mod lapinfo;
mod realtime_car_update;
mod realtime_update;
mod registration;
mod track_data;

pub use broadcasting_event::*;
pub use data::*;
pub use entrylist::*;
pub use lapinfo::*;
use num_enum::FromPrimitive;
pub use realtime_car_update::*;
pub use realtime_update::*;
pub use track_data::*;

use crate::{StateChange, Weather};

use self::registration::{RegisterConnection, RegistrationResult};

pub struct BroadcastState {
    connection_id: i32,
    _entry_list: EntryList,
    track_data: TrackData,
    realtime_update: RealtimeUpdate,
    tx: UnboundedSender<StateChange>,
}

impl BroadcastState {
    pub fn new(tx: UnboundedSender<StateChange>) -> BroadcastState {
        BroadcastState {
            connection_id: 0,
            _entry_list: Default::default(),
            track_data: Default::default(),
            realtime_update: Default::default(),
            tx,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.run_inner() {
                Ok(()) => {}
                Err(e) => {
                    error!("failed to connect to udp: {:?}", e);
                    std::thread::sleep(Duration::from_millis(1000))
                }
            }
        }
    }

    pub fn run_inner(&mut self) -> io::Result<()> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        socket.connect("127.0.0.1:9000").unwrap();

        let request_connection = RegisterConnection {
            display_name: "Vapor Manager".to_string(),
            connection_password: "asd".to_string(),
            ms_realtime_update_interval: 250,
            command_password: "".to_string(),
        };

        socket.send(&request_connection.serialize())?;

        loop {
            let mut buf = [0; 2056];
            let _amt = socket.recv(&mut buf)?;

            let input = &buf[..];
            let (input, msg_type) = InboundMessageTypes::read(input).unwrap();

            match msg_type {
                InboundMessageTypes::RegistrationResult => {
                    let (_input, reg_res) = RegistrationResult::deserialize(input).unwrap();
                    debug!("{:?}", reg_res);
                    if !reg_res.connection_success {
                        break;
                    }

                    self.connection_id = reg_res.id;

                    let req_track = RequestTrackData::new(self.connection_id).serialize();
                    socket.send(&req_track)?;
                }
                InboundMessageTypes::RealtimeUpdate => {
                    let (_input, update) = RealtimeUpdate::deserialize(input).unwrap();
                    if self.realtime_update.ambient_temp != update.ambient_temp
                        || self.realtime_update.track_temp != update.track_temp
                        || self.realtime_update.clouds != update.clouds
                        || self.realtime_update.rain_level != update.rain_level
                        || self.realtime_update.wetness != update.wetness
                    {
                        self.tx
                            .unbounded_send(StateChange::Weather(Weather {
                                ambient_temp: update.ambient_temp,
                                track_temp: update.track_temp,
                                clouds: update.clouds,
                                rain_level: update.rain_level,
                                wetness: update.wetness,
                            }))
                            .unwrap();
                    }

                    self.realtime_update = update;
                }
                InboundMessageTypes::RealtimeCarUpdate => (),
                InboundMessageTypes::EntryList => (),
                InboundMessageTypes::EntryListCar => (),
                InboundMessageTypes::TrackData => {
                    let (_input, track_data) = TrackData::deserialize(input).unwrap();
                    debug!("{:?}", track_data);
                    self.tx
                        .unbounded_send(StateChange::TrackName(track_data.name.clone()))
                        .unwrap();
                    self.track_data = track_data;
                }
                InboundMessageTypes::BroadcastingEvent => (),
                InboundMessageTypes::ERROR => {
                    error!("unknown message type received")
                }
            }
        }

        Ok(())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum OutboundMessageTypes {
    RegisterCommandApplication = 1,
    UnregisterCommandApplication = 9,

    RequestEntryList = 10,
    RequestTrackData = 11,

    ChangeHudPage = 49,
    ChangeFocus = 50,
    InstantReplayRequest = 51,

    PlayManualReplayHighlight = 52, // TODO, but planned
    SaveManualReplayHighlight = 60, // TODO, but planned: saving manual replays gives distributed clients the possibility to see the play the same replay
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, FromPrimitive)]
pub enum InboundMessageTypes {
    RegistrationResult = 1,
    RealtimeUpdate = 2,
    RealtimeCarUpdate = 3,
    EntryList = 4,
    EntryListCar = 6,
    TrackData = 5,
    BroadcastingEvent = 7,
    #[default]
    ERROR = 0,
}

impl InboundMessageTypes {
    fn read(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, msg_type) = u8(input)?;
        Ok((input, Self::from_primitive(msg_type)))
    }
}

pub trait BroadcastNetworkProtocolInbound {
    const TYPE: InboundMessageTypes;
    const PROTOCOL_VERSION: u8 = 4;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

pub trait BroadcastNetworkProtocolOutbound {
    const TYPE: OutboundMessageTypes;
    const PROTOCOL_VERSION: u8 = 4;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

pub fn read_string(input: &[u8]) -> IResult<&[u8], &str> {
    let (input, length) = le_u16(input)?;
    let (input, s) = take(length as usize)(input)?;
    let s = core::str::from_utf8(s).unwrap();

    Ok((input, s))
}

pub fn write_string(input: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let bytes = input.bytes();
    vec.extend((bytes.len() as u16).to_le_bytes());
    vec.extend(bytes);

    vec
}

pub struct RequestEntryList {
    connection_id: i32,
}

impl RequestEntryList {
    pub fn new(id: i32) -> Self {
        RequestEntryList { connection_id: id }
    }
}

impl BroadcastNetworkProtocolOutbound for RequestEntryList {
    const TYPE: OutboundMessageTypes = OutboundMessageTypes::RequestEntryList;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.push(Self::TYPE as u8);
        out.extend(self.connection_id.to_le_bytes());

        out
    }

    fn deserialize(_input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

pub struct RequestTrackData {
    connection_id: i32,
}

impl RequestTrackData {
    pub fn new(id: i32) -> Self {
        RequestTrackData { connection_id: id }
    }
}

impl BroadcastNetworkProtocolOutbound for RequestTrackData {
    const TYPE: OutboundMessageTypes = OutboundMessageTypes::RequestTrackData;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.push(Self::TYPE as u8);
        out.extend(self.connection_id.to_le_bytes());

        out
    }

    fn deserialize(_input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
