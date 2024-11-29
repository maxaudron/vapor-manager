use tokio_util::{
    bytes::BytesMut,
    codec::{Decoder, Encoder},
};

use super::{
    registration::RegistrationResult, BroadcastInboundMessage, BroadcastNetworkProtocolInbound,
    BroadcastNetworkProtocolOutbound, BroadcastOutboundMessage, BroadcastingEvent, CarInfo,
    EntryList, InboundMessageTypes, OutboundMessageTypes, RealtimeCarUpdate, RealtimeUpdate,
    TrackData,
};

pub struct BroadcastCodec {}

impl BroadcastCodec {
    pub fn new() -> BroadcastCodec {
        BroadcastCodec {}
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FramedError {
    #[error("recoverable nom parsing error")]
    NomError,
    #[error("non-recoverable nom parsing failure")]
    NomFailure,
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

fn err<T>(e: nom::Err<nom::error::Error<&[u8]>>) -> Result<Option<T>, FramedError> {
    match e {
        nom::Err::Incomplete(_) => Ok(None),
        nom::Err::Error(_) => Err(FramedError::NomError),
        nom::Err::Failure(_) => Err(FramedError::NomFailure),
    }
}

impl Decoder for BroadcastCodec {
    type Item = BroadcastInboundMessage;

    type Error = FramedError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let Some((input, msg_type)) = InboundMessageTypes::read(src)
            .map(|x| Some(x))
            .or_else(err)?
        else {
            return Ok(None);
        };

        match msg_type {
            InboundMessageTypes::RegistrationResult => {
                let Some(msg) = RegistrationResult::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::RegistrationResult(msg)))
            }
            InboundMessageTypes::RealtimeUpdate => {
                let Some(msg) = RealtimeUpdate::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::RealtimeUpdate(msg)))
            }
            InboundMessageTypes::RealtimeCarUpdate => {
                let Some(msg) = RealtimeCarUpdate::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::RealtimeCarUpdate(msg)))
            }
            InboundMessageTypes::EntryList => {
                let Some(msg) = EntryList::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::EntryList(msg)))
            }
            InboundMessageTypes::EntryListCar => {
                let Some(msg) = CarInfo::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::EntryListCar(msg)))
            }
            InboundMessageTypes::TrackData => {
                let Some(msg) = TrackData::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::TrackData(msg)))
            }
            InboundMessageTypes::BroadcastingEvent => {
                let Some(msg) = BroadcastingEvent::deserialize(input)
                    .map(|x| Some(x.1))
                    .or_else(err)?
                else {
                    return Ok(None);
                };
                src.clear();
                Ok(Some(BroadcastInboundMessage::BroadcastingEvent(msg)))
            }
            InboundMessageTypes::ERROR => Ok(None),
        }
    }
}

impl Encoder<BroadcastOutboundMessage> for BroadcastCodec {
    type Error = std::io::Error;

    fn encode(
        &mut self,
        item: BroadcastOutboundMessage,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        match item {
            BroadcastOutboundMessage::RegisterCommandApplication(i) => dst.extend(i.serialize()),
            BroadcastOutboundMessage::UnregisterCommandApplication => {
                dst.extend([OutboundMessageTypes::UnregisterCommandApplication as u8])
            }
            BroadcastOutboundMessage::RequestEntryList(i) => dst.extend(i.serialize()),
            BroadcastOutboundMessage::RequestTrackData(i) => dst.extend(i.serialize()),
        }

        Ok(())
    }
}
