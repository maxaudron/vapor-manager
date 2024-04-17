use nom::{
    bytes::complete::take,
    number::complete::{le_u16, u8},
    IResult,
};

mod runtime;
pub use runtime::*;
#[cfg(debug_assertions)]
mod debugger;
#[cfg(debug_assertions)]
pub use debugger::*;

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

use self::registration::RegistrationResult;

pub enum BroadcastInboundMessage {
    RegistrationResult(RegistrationResult),
    RealtimeUpdate(RealtimeUpdate),
    RealtimeCarUpdate(RealtimeCarUpdate),
    EntryList(EntryList),
    EntryListCar(CarInfo),
    TrackData(TrackData),
    BroadcastingEvent(BroadcastingEvent),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, FromPrimitive)]
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

    #[default]
    Error = 0,
}

impl OutboundMessageTypes {
    fn read(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, msg_type) = u8(input)?;
        Ok((input, Self::from_primitive(msg_type)))
    }
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

fn disconnect() -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.push(OutboundMessageTypes::UnregisterCommandApplication as u8);

    out
}
