use nom::{
    number::complete::{le_i32, u8},
    sequence::tuple,
    IResult,
};
use serde::{Deserialize, Serialize};

use super::{
    read_string, write_string, BroadcastNetworkProtocolInbound, BroadcastNetworkProtocolOutbound,
    InboundMessageTypes, OutboundMessageTypes,
};

/// Will try to register this client in the targeted ACC instance.
/// Needs to be called once, before anything else can happen.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterConnection {
    pub display_name: String,
    pub connection_password: String,
    pub ms_realtime_update_interval: i32,
    pub command_password: String,
}

impl BroadcastNetworkProtocolOutbound for RegisterConnection {
    const TYPE: OutboundMessageTypes = OutboundMessageTypes::RegisterCommandApplication;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.push(Self::TYPE as u8);
        out.push(Self::PROTOCOL_VERSION as u8);
        out.extend(write_string(&self.display_name));
        out.extend(write_string(&self.connection_password));
        out.extend(self.ms_realtime_update_interval.to_le_bytes());
        out.extend(write_string(&self.command_password));

        out
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Self> {
        let (
            input,
            (display_name, connection_password, ms_realtime_update_interval, command_password),
        ) = tuple((read_string, read_string, le_i32, read_string))(input)?;

        Ok((
            input,
            Self {
                display_name: display_name.to_owned(),
                connection_password: connection_password.to_owned(),
                ms_realtime_update_interval,
                command_password: command_password.to_owned(),
            },
        ))
    }
}

/// Response to the initial connection
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationResult {
    pub id: i32,
    pub connection_success: bool,
    pub read_only: bool,
    pub err_msg: String,
}

impl BroadcastNetworkProtocolInbound for RegistrationResult {
    const TYPE: InboundMessageTypes = InboundMessageTypes::RegistrationResult;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.id.to_le_bytes());
        out.push(self.connection_success as u8);
        out.push(self.read_only as u8);
        out.extend(write_string(&self.err_msg));

        out
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, (id, connection_success, read_only, err_msg)) =
            tuple((le_i32, u8, u8, read_string))(input)?;

        Ok((
            input,
            Self {
                id,
                connection_success: connection_success > 0,
                read_only: read_only == 0,
                err_msg: err_msg.to_owned(),
            },
        ))
    }
}
