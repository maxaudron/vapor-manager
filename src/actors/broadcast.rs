use std::net::{Ipv4Addr, SocketAddr};

use actix::prelude::*;
use futures::stream::SplitSink;
use futures_util::StreamExt;
use io::{SinkWrite, WriteHandler};
use tokio::net::UdpSocket;
use tokio_util::udp::UdpFramed;
use tracing::{debug, error, instrument, trace};

use crate::{
    telemetry::broadcast::{
        BroadcastCodec, BroadcastInboundMessage, BroadcastOutboundMessage, FramedError,
        RegisterConnection, RequestTrackData,
    },
    StateChange, PROGRAM_NAME,
};

use super::{Router, ShmGameState};

pub struct Broadcast {
    id: i32,
    router: Addr<Router>,
    sink: Option<
        SinkWrite<
            (BroadcastOutboundMessage, SocketAddr),
            SplitSink<UdpFramed<BroadcastCodec>, (BroadcastOutboundMessage, SocketAddr)>,
        >,
    >,
}

impl Broadcast {
    pub fn new(router: Addr<Router>) -> Broadcast {
        Broadcast {
            router,
            sink: None,
            id: 0,
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
        self.sink
            .as_mut()
            .unwrap()
            .write((msg, *SOCKET_ADDR))
            .unwrap();
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
                        self.id = r.id;
                        // self.router.send(BroadcastConnected);
                        ctx.address()
                            .do_send(BroadcastOutboundMessage::RequestTrackData(
                                RequestTrackData::new(self.id),
                            ));
                    } else {
                        error!("failed to register to broadcast api: {:?}", r.err_msg);
                        // self.router.send(BroadcastDisconnected);
                    };
                }
                BroadcastInboundMessage::RealtimeUpdate(_) => (),
                BroadcastInboundMessage::RealtimeCarUpdate(_) => (),
                BroadcastInboundMessage::EntryList(_) => (),
                BroadcastInboundMessage::EntryListCar(_) => (),
                BroadcastInboundMessage::TrackData(d) => {
                    self.router.do_send(StateChange::TrackName(d.name));
                }
                BroadcastInboundMessage::BroadcastingEvent(_) => (),
            },
            Err(e) => error!("got framed error: {:?}", e),
        };
    }
}
