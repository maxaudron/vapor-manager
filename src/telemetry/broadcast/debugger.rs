use std::sync::Arc;

use dioxus::hooks::UnboundedReceiver;
use futures_util::StreamExt;
use tokio::net::UdpSocket;
use tracing::debug;

use crate::telemetry::broadcast::{
    registration::RegisterConnection, BroadcastNetworkProtocolOutbound, RequestEntryList, RequestTrackData,
};

use super::{BroadcastInboundMessage, BroadcastNetworkProtocolInbound, OutboundMessageTypes};

pub struct BroadcastDebugger;

impl BroadcastDebugger {
    pub async fn coroutine(mut rx: UnboundedReceiver<BroadcastInboundMessage>) {
        loop {
            let socket = Arc::new(UdpSocket::bind(("127.0.0.1", 9000)).await.unwrap());

            let mut addr = None;
            loop {
                let mut buf = [0; 2048];
                tokio::select! {
                    msg = rx.next() => {
                        if let Some(addr) = addr {
                            if let Some(msg) = msg {
                                match msg {
                                    BroadcastInboundMessage::RegistrationResult(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                    BroadcastInboundMessage::RealtimeUpdate(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                    BroadcastInboundMessage::RealtimeCarUpdate(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                    BroadcastInboundMessage::EntryList(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                    BroadcastInboundMessage::EntryListCar(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                    BroadcastInboundMessage::TrackData(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                    BroadcastInboundMessage::BroadcastingEvent(v) => socket.send_to(&v.serialize(), addr).await.unwrap(),
                                };
                            }
                        }
                    }
                    recv = socket.recv_from(&mut buf) => {
                        let (_amt, recv_addr) = recv.unwrap();

                        let input = &buf[..];
                        let (input, msg_type) = OutboundMessageTypes::read(input).unwrap();

                        match msg_type {
                            OutboundMessageTypes::RegisterCommandApplication => {
                                let (_, msg) = RegisterConnection::deserialize(input).unwrap();
                                addr = Some(recv_addr);
                                debug!("received outbound message: {msg:?}");
                            },
                            OutboundMessageTypes::UnregisterCommandApplication => {
                                debug!("received UnregisterCommandApplication");
                            },
                            OutboundMessageTypes::RequestEntryList => {
                                let (_, msg) = RequestEntryList::deserialize(input).unwrap();
                                debug!("received outbound message: {msg:?}");
                            },
                            OutboundMessageTypes::RequestTrackData => {
                                let (_, msg) = RequestTrackData::deserialize(input).unwrap();
                                debug!("received outbound message: {msg:?}");
                            },
                            OutboundMessageTypes::ChangeHudPage => todo!(),
                            OutboundMessageTypes::ChangeFocus => todo!(),
                            OutboundMessageTypes::InstantReplayRequest => todo!(),
                            OutboundMessageTypes::PlayManualReplayHighlight => todo!(),
                            OutboundMessageTypes::SaveManualReplayHighlight => todo!(),
                            OutboundMessageTypes::Error => todo!(),
                        }
                    }
                }
            }
        }
    }
}
