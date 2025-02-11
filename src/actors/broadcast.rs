use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use actix::prelude::*;
use futures::stream::SplitSink;
use futures_util::StreamExt;
use io::{SinkWrite, WriteHandler};
use tokio::net::UdpSocket;
use tokio_util::udp::UdpFramed;
use tracing::{debug, error, instrument, trace};

use crate::{
    actors::{
        fuel_calculator::FuelMessage,
        ui::{LapTimeData, UiUpdate},
    },
    telemetry::{
        broadcast::{
            BroadcastCodec, BroadcastInboundMessage, BroadcastOutboundMessage, FramedError, RaceSessionType,
            RealtimeCarUpdate, RealtimeUpdate, RegisterConnection, RequestTrackData, TrackData,
        },
        LapTime,
    },
    PROGRAM_NAME,
};

use super::{
    setup_manager::SetupChange,
    ui::{SessionInfo, Weather},
    Reset, Router, ShmGameState,
};

pub struct Broadcast {
    id: i32,
    car_id: i16,
    router: Addr<Router>,
    sink: Option<
        SinkWrite<
            (BroadcastOutboundMessage, SocketAddr),
            SplitSink<UdpFramed<BroadcastCodec>, (BroadcastOutboundMessage, SocketAddr)>,
        >,
    >,

    session_info: SessionInfo,

    track_data: TrackData,
    realtime_update: RealtimeUpdate,
    realtime_car_update: RealtimeCarUpdate,

    avg_lap_count: u32,
    avg_lap_time: LapTime,
}

impl Broadcast {
    pub fn new(router: Addr<Router>) -> Broadcast {
        Broadcast {
            router,
            sink: None,
            id: 0,
            car_id: 0,
            session_info: Default::default(),
            track_data: Default::default(),
            realtime_update: Default::default(),
            realtime_car_update: Default::default(),
            avg_lap_count: 0,
            avg_lap_time: Default::default(),
        }
    }
}

static SOCKET_ADDR: std::sync::LazyLock<SocketAddr> =
    std::sync::LazyLock::new(|| SocketAddr::new("127.0.0.1".parse().unwrap(), 9000));

impl Actor for Broadcast {
    type Context = Context<Self>;

    #[instrument(level = "debug", skip_all)]
    fn started(&mut self, ctx: &mut Self::Context) {
        let connect = async move {
            let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await.unwrap();
            socket.connect("127.0.0.1:9000").await.unwrap();
            debug!("setup udp connection");

            UdpFramed::new(socket, BroadcastCodec::new()).split()
        };

        connect
            .into_actor(self)
            .map(|(sink, stream), actor, ctx| {
                ctx.add_stream(stream);
                actor.sink = Some(SinkWrite::new(sink, ctx));
                debug!("commited stream and sink");
            })
            .spawn(ctx);
    }

    #[instrument(level = "debug", skip_all)]
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("unregistering from broadcast API & stopping actor");
        self.sink
            .as_mut()
            .unwrap()
            .write((
                BroadcastOutboundMessage::UnregisterCommandApplication,
                *SOCKET_ADDR,
            ))
            .unwrap();
    }
}

impl WriteHandler<std::io::Error> for Broadcast {}

impl Handler<BroadcastOutboundMessage> for Broadcast {
    type Result = ();

    #[instrument(skip(self, _ctx))]
    fn handle(&mut self, msg: BroadcastOutboundMessage, _ctx: &mut Self::Context) -> Self::Result {
        debug!("sending outbound message to socket");
        self.sink.as_mut().unwrap().write((msg, *SOCKET_ADDR)).unwrap();
    }
}

impl Handler<ShmGameState> for Broadcast {
    type Result = ();

    #[instrument(skip(self, ctx))]
    fn handle(&mut self, msg: ShmGameState, ctx: &mut Self::Context) -> Self::Result {
        debug!("got msg: {:?}", msg);
        match msg {
            ShmGameState::Disconnected => ctx
                .address()
                .do_send(BroadcastOutboundMessage::UnregisterCommandApplication),
            ShmGameState::Connected => {
                let request_connection = RegisterConnection {
                    display_name: PROGRAM_NAME.to_string(),
                    connection_password: "asd".to_string(),
                    ms_realtime_update_interval: 250,
                    command_password: "".to_string(),
                };

                ctx.address()
                    .do_send(BroadcastOutboundMessage::RegisterCommandApplication(
                        request_connection,
                    ));
            }
        };
    }
}

impl Handler<Reset> for Broadcast {
    type Result = ();

    fn handle(&mut self, _msg: Reset, _ctx: &mut Self::Context) -> Self::Result {
        self.reset();
    }
}

impl StreamHandler<Result<(BroadcastInboundMessage, SocketAddr), FramedError>> for Broadcast {
    #[instrument(skip_all)]
    fn handle(
        &mut self,
        item: Result<(BroadcastInboundMessage, SocketAddr), FramedError>,
        ctx: &mut Self::Context,
    ) {
        trace!("received broadcast message: {:?}", item);

        match item {
            Ok(msg) => match msg.0 {
                BroadcastInboundMessage::RegistrationResult(r) => {
                    if r.connection_success {
                        self.reset();
                        self.id = r.id;

                        self.router
                            .send(crate::actors::telemetry::CarID)
                            .into_actor(self)
                            .map(|car_id, actor, _ctx| {
                                actor.car_id = car_id.unwrap();
                                debug!("retrived car id: {}", car_id.unwrap());
                            })
                            .spawn(ctx);

                        ctx.address().do_send(BroadcastOutboundMessage::RequestTrackData(
                            RequestTrackData::new(self.id),
                        ));

                        self.router.do_send(UiUpdate::SessionLive(true));
                        self.router.do_send(crate::actors::Reset);
                    } else {
                        error!("failed to register to broadcast api: {:?}", r.err_msg);
                        self.router.do_send(UiUpdate::SessionLive(false));
                    };
                }
                BroadcastInboundMessage::RealtimeUpdate(d) => {
                    self.update_weather(&d);
                    self.update_time(&d);

                    self.realtime_update = d;
                }
                BroadcastInboundMessage::RealtimeCarUpdate(update) => {
                    if update.car_index == self.car_id {
                        self.update_laps(update)
                    }
                }
                BroadcastInboundMessage::EntryList(_) => (),
                BroadcastInboundMessage::EntryListCar(_) => (),
                BroadcastInboundMessage::TrackData(d) => {
                    self.update_track_name(&d);

                    self.track_data = d;
                }
                BroadcastInboundMessage::BroadcastingEvent(_) => (),
            },
            Err(e) => error!("got framed error: {:?}", e),
        };
    }
}

impl Broadcast {
    fn update_track_name(&mut self, update: &TrackData) {
        if self.track_data.name != update.name {
            self.session_info.name = update.name.clone();
            self.router.do_send(UiUpdate::TrackName(update.name.clone()));
        }
    }
    fn update_weather(&mut self, update: &RealtimeUpdate) {
        if self.realtime_update.ambient_temp != update.ambient_temp
            || self.realtime_update.track_temp != update.track_temp
            || self.realtime_update.clouds != update.clouds
            || self.realtime_update.rain_level != update.rain_level
            || self.realtime_update.wetness != update.wetness
        {
            self.session_info.weather = Weather {
                ambient_temp: update.ambient_temp,
                track_temp: update.track_temp,
                clouds: update.clouds,
                rain_level: update.rain_level,
                wetness: update.wetness,
            };

            self.router.do_send(UiUpdate::Weather(self.session_info.weather));
            self.router
                .do_send(SetupChange::Weather(self.session_info.weather));
        }
    }

    fn update_time(&mut self, update: &RealtimeUpdate) {
        if self.realtime_update.session_length() != update.session_length() {
            debug!("session time: {:?}", update.session_length());
            match update.session_type {
                RaceSessionType::Qualifying | RaceSessionType::Superpole => {
                    self.router
                        .do_send(FuelMessage::QualiLength(Duration::from_millis(
                            update.session_length().ceil() as u64,
                        )))
                }
                RaceSessionType::Race | RaceSessionType::Hotstint => self.router.do_send(
                    FuelMessage::RaceLength(Duration::from_millis(update.session_length().ceil() as u64)),
                ),
                _ => (),
            };

            self.realtime_update.session_time = update.session_time;
            self.realtime_update.session_end_time = update.session_end_time;
        }
    }

    fn update_laps(&mut self, update: RealtimeCarUpdate) {
        if self.realtime_car_update.laps != update.laps {
            debug!("realtime update: {:?}", update);
            self.realtime_car_update.laps = update.laps;

            if update.laps >= 1 {
                let last = update.last_lap;

                // Calculate Avg Lap Time and send to fuel calculator
                if !last.invalid {
                    let avg = ((self.avg_lap_time.duration().as_millis() * self.avg_lap_count as u128)
                        + last.laptime.unwrap() as u128)
                        / (self.avg_lap_count as u128 + 1);

                    self.avg_lap_count += 1;
                    self.avg_lap_time = Duration::from_millis(avg as u64).into();
                    self.router
                        .do_send(FuelMessage::AvgLapTime(self.avg_lap_time.clone()));
                }

                self.router.do_send(UiUpdate::LapTime(LapTimeData {
                    number: update.laps as i32,
                    sectors: last
                        .splits
                        .iter()
                        .filter_map(|s| *s)
                        .map(|s| Duration::from_millis(s as u64).into())
                        .collect(),
                    lap_type: last.lap_type,
                    time: Duration::from_millis(last.laptime.unwrap() as u64).into(),
                    valid: !last.invalid,
                }));
            }
        }
    }

    /// Reset the consumable fields for reuse in a new session
    fn reset(&mut self) {
        self.avg_lap_count = 0;
        self.avg_lap_time = Default::default();

        self.session_info = Default::default();

        self.track_data = Default::default();
        self.realtime_update = Default::default();
        self.realtime_car_update = Default::default();
    }
}
