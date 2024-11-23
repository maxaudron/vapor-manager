use actix::prelude::*;
use tracing::debug;

use crate::{setup::SetupChange, StateChange};

use super::{broadcast::Broadcast, telemetry::Telemetry};

pub struct Router {
    telemetry: Addr<Telemetry>,
    broadcast: Addr<Broadcast>,
}

impl Router {
    pub fn initialize(arbiter: ArbiterHandle) -> actix::Addr<Router> {
        Router::start_in_arbiter(&arbiter, |ctx| {
            let telemetry = Telemetry::new(ctx.address());
            let telemetry_arb = Arbiter::new();
            let telemetry = Telemetry::start_in_arbiter(&telemetry_arb.handle(), |_| telemetry);

            let broadcast = Broadcast::new(ctx.address()).start();

            Router {
                telemetry,
                broadcast,
            }
        })
    }
}

impl Actor for Router {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("router started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("router stopped");
    }
}

#[derive(Debug, Clone, Copy, Message)]
#[rtype(result = "()")]
pub enum ShmGameState {
    Disconnected,
    Connected,
}

impl Handler<ShmGameState> for Router {
    type Result = ();

    fn handle(&mut self, msg: ShmGameState, ctx: &mut Self::Context) -> Self::Result {
        debug!(handler = "ShmGameState", msg = ?msg);

        self.broadcast.do_send(msg);

        match msg {
            ShmGameState::Disconnected => (),
            ShmGameState::Connected => {}
        }
    }
}

impl Handler<SetupChange> for Router {
    type Result = ();

    fn handle(&mut self, msg: SetupChange, ctx: &mut Self::Context) -> Self::Result {
        debug!(msg = ?msg)
    }
}

impl Handler<StateChange> for Router {
    type Result = ();

    fn handle(&mut self, msg: StateChange, ctx: &mut Self::Context) -> Self::Result {
        debug!(msg = ?msg)
    }
}
