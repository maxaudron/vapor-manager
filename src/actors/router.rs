use std::sync::OnceLock;

use actix::prelude::*;
use tracing::debug;

use crate::{setup::SetupChange, StateChange};

pub struct Router {
    telemetry: super::telemetry::Telemetry,
}

impl Router {
    pub fn new(telemetry: super::telemetry::Telemetry) -> Router {
        Router { telemetry }
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

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum ShmGameState {
    Disconnected,
    Connected,
}

impl Handler<ShmGameState> for Router {
    type Result = ();

    fn handle(&mut self, msg: ShmGameState, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<SetupChange> for Router {
    type Result = ();

    fn handle(&mut self, msg: SetupChange, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<StateChange> for Router {
    type Result = ();

    fn handle(&mut self, msg: StateChange, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
