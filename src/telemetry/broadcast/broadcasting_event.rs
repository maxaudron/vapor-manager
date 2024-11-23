use nom::{
    number::streaming::{le_i32, u8},
    sequence::tuple,
};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::telemetry::broadcast::read_string;

use super::{BroadcastNetworkProtocolInbound, BroadcastingCarEventType, InboundMessageTypes};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastingEvent {
    pub event_type: BroadcastingCarEventType,
    pub msg: String,
    pub time_ms: i32,
    pub car_id: i32,
}

impl BroadcastNetworkProtocolInbound for BroadcastingEvent {
    const TYPE: InboundMessageTypes = InboundMessageTypes::BroadcastingEvent;

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, (event_type, msg, time_ms, car_id)) =
            tuple((u8, read_string, le_i32, le_i32))(input)?;

        Ok((
            input,
            Self {
                event_type: BroadcastingCarEventType::from_primitive(event_type),
                msg: msg.to_owned(),
                time_ms,
                car_id,
            },
        ))
    }
}
