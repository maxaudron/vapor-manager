use dioxus::hooks::{UnboundedReceiver, UnboundedSender};
use futures_util::StreamExt;
use std::{io, net::Ipv4Addr, sync::Arc};
use tokio::net::UdpSocket;
use tracing::{debug, error};

use super::{
    disconnect,
    registration::{RegisterConnection, RegistrationResult},
    BroadcastNetworkProtocolInbound, BroadcastNetworkProtocolOutbound, EntryList,
    InboundMessageTypes, RealtimeUpdate, RequestTrackData, TrackData,
};
use crate::{setup::SetupChange, StateChange, Weather};

pub struct BroadcastState {
    connection_id: i32,
    _entry_list: EntryList,
    track_data: TrackData,
    realtime_update: RealtimeUpdate,
    state_tx: UnboundedSender<StateChange>,
    setup_tx: UnboundedSender<SetupChange>,
    socket: Option<Arc<UdpSocket>>,
    broadcast_rx: UnboundedReceiver<BroadcastMsg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BroadcastMsg {
    Connect,
    Disconnect,
    Aborted,
}

impl BroadcastState {
    pub fn new(
        state_tx: UnboundedSender<StateChange>,
        setup_tx: UnboundedSender<SetupChange>,
        broadcast_rx: UnboundedReceiver<BroadcastMsg>,
    ) -> BroadcastState {
        BroadcastState {
            connection_id: 0,
            _entry_list: Default::default(),
            track_data: Default::default(),
            realtime_update: Default::default(),
            state_tx,
            setup_tx,
            socket: None,
            broadcast_rx,
        }
    }

    pub async fn run(&mut self) {
        debug!("starting acc broadcast api connection");
        let state_tx = self.state_tx.clone();

        match self.run_inner().await {
            Ok(()) => (),
            Err(e) => {
                error!("failed to connect to udp: {:?}", e);
            }
        }

        debug!("acc broadcast api connection has stopped");
        state_tx
            .unbounded_send(StateChange::BroadcastConnected(false))
            .unwrap();
    }

    pub async fn run_inner(&mut self) -> io::Result<()> {
        let socket = Arc::new(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await.unwrap());
        self.socket = Some(socket.clone());
        socket.connect("127.0.0.1:9000").await.unwrap();

        let request_connection = RegisterConnection {
            display_name: "Vapor Manager".to_string(),
            connection_password: "asd".to_string(),
            ms_realtime_update_interval: 250,
            command_password: "".to_string(),
        };

        socket.send(&request_connection.serialize()).await?;

        loop {
            let mut buf = [0; 2056];

            tokio::select! {
                msg = self.broadcast_rx.next() => {
                    if let Some(msg) = msg {
                        match msg {
                            BroadcastMsg::Disconnect => {
                                socket.send(&disconnect()).await?;
                                return Ok(());
                            }
                            _ => (),
                        }
                    }
                }
                amt = socket.recv(&mut buf) => {
                    let _amt = amt?;

                    let input = &buf[..];
                    let (input, msg_type) = InboundMessageTypes::read(input).unwrap();

                    match msg_type {
                        InboundMessageTypes::RegistrationResult => {
                            let (_input, reg_res) = RegistrationResult::deserialize(input).unwrap();
                            debug!("{:?}", reg_res);
                            if !reg_res.connection_success {
                                break;
                            }

                            self.state_tx
                                .unbounded_send(StateChange::BroadcastConnected(true))
                                .unwrap();
                            self.connection_id = reg_res.id;

                            let req_track = RequestTrackData::new(self.connection_id).serialize();
                            socket.send(&req_track).await?;
                        }
                        InboundMessageTypes::RealtimeUpdate => {
                            let (_input, update) = RealtimeUpdate::deserialize(input).unwrap();
                            if self.realtime_update.ambient_temp != update.ambient_temp
                                || self.realtime_update.track_temp != update.track_temp
                                || self.realtime_update.clouds != update.clouds
                                || self.realtime_update.rain_level != update.rain_level
                                || self.realtime_update.wetness != update.wetness
                            {
                                let weather = Weather {
                                    ambient_temp: update.ambient_temp,
                                    track_temp: update.track_temp,
                                    clouds: update.clouds,
                                    rain_level: update.rain_level,
                                    wetness: update.wetness,
                                };

                                self.state_tx
                                    .unbounded_send(StateChange::Weather(weather.clone()))
                                    .unwrap();
                                self.setup_tx
                                    .unbounded_send(SetupChange::Weather(weather))
                                    .unwrap();
                            }

                            if self.realtime_update.session_length() != update.session_length() {
                                debug!("session time: {:?}", update.session_length());
                                self.setup_tx
                                    .unbounded_send(SetupChange::SessionLength((
                                        update.session_type,
                                        std::time::Duration::from_millis(update.session_length().round() as u64),
                                    )))
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
                            self.state_tx
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
            }
        }

        Ok(())
    }
}
