use actix::prelude::*;
use tracing::debug;

use super::{
    broadcast::Broadcast,
    setup_manager::{SetupChange, SetupManager},
    telemetry::Telemetry,
    ui::{UiState, UiUpdate},
};

pub struct Router {
    #[allow(unused)]
    telemetry: Addr<Telemetry>,
    broadcast: Addr<Broadcast>,
    setup_manager: Addr<SetupManager>,
    clients: Vec<Addr<UiState>>,
}

impl Router {
    pub fn initialize(arbiter: ArbiterHandle) -> actix::Addr<Router> {
        Router::start_in_arbiter(&arbiter, |ctx| {
            let telemetry = Telemetry::new(ctx.address());
            let telemetry_arb = Arbiter::new();
            let telemetry = Telemetry::start_in_arbiter(&telemetry_arb.handle(), |_| telemetry);

            let broadcast = Broadcast::new(ctx.address()).start();

            let setup_manager = SetupManager::new(ctx.address()).start();

            Router {
                telemetry,
                broadcast,
                setup_manager,
                clients: Vec::new(),
            }
        })
    }

    fn send_clients(&mut self, msg: UiUpdate) {
        self.clients.iter_mut().for_each(|c| c.do_send(msg.clone()));
    }
}

impl Actor for Router {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("router started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
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

    fn handle(&mut self, msg: ShmGameState, _ctx: &mut Self::Context) -> Self::Result {
        debug!(handler = "ShmGameState", msg = ?msg);

        self.broadcast.do_send(msg);

        match msg {
            ShmGameState::Disconnected => self.send_clients(UiUpdate::SessionLive(false)),
            ShmGameState::Connected => {}
        }
    }
}

impl Handler<UiUpdate> for Router {
    type Result = ();

    fn handle(&mut self, msg: UiUpdate, _ctx: &mut Self::Context) -> Self::Result {
        debug!(name = "sending msg to clients", msg = ?msg);
        self.send_clients(msg);
    }
}

impl Handler<SetupChange> for Router {
    type Result = ();

    fn handle(&mut self, msg: SetupChange, _ctx: &mut Self::Context) -> Self::Result {
        debug!(name = "sending msg to setup_manager", msg = ?msg);
        self.setup_manager.do_send(msg);
    }
}

impl Handler<ClientManagement> for Router {
    type Result = ();

    fn handle(&mut self, msg: ClientManagement, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClientManagement::Add(client) => {
                debug!("adding client");
                self.clients.push(client)
            }
            ClientManagement::Remove(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum ClientManagement {
    Add(Addr<UiState>),
    #[allow(unused)]
    Remove(Addr<UiState>),
}
