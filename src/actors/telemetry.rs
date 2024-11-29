use std::time::Duration;

use actix::prelude::*;
use tracing::{debug, error, warn};

use crate::{
    actors::ui::{UiState, UiUpdate},
    setup,
    telemetry::{
        self, Graphics, LapHistory, PageFileGraphics, PageFilePhysics, PageFileStatic, Physics,
        SharedMemoryPage, StaticData, Status, TelemetryError,
    },
    StateChange,
};

use super::Router;

pub struct Telemetry {
    interval: SpawnHandle,
    router: Addr<Router>,
    connected: bool,

    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,

    pub lap_history: LapHistory,
    pub lap_result: telemetry::LapWheels,
}

impl Telemetry {
    pub fn new(router: Addr<Router>) -> Self {
        Self {
            router,
            interval: Default::default(),
            connected: Default::default(),
            static_data: Default::default(),
            physics: Default::default(),
            graphics: Default::default(),
            lap_history: Default::default(),
            lap_result: Default::default(),
        }
    }

    /// Commit the update to data to be used in the next round
    fn commit_update(&mut self, update: TelemetryUpdate) {
        self.static_data = update.static_data;

        if update.graphics.packet_id > self.graphics.packet_id {
            self.graphics = update.graphics.clone();
            self.lap_history.h_graphics.push(update.graphics);
        }

        // TODO this might be causing the problems with the repeated lap times
        // what happens when this number reaches it's max?
        if update.physics.packet_id > self.physics.packet_id {
            self.physics = update.physics.clone();
            self.lap_history.h_physics.push(update.physics);
        }
    }

    /// Read newest values from memory shim
    fn get_update() -> Result<TelemetryUpdate, TelemetryError> {
        let static_data = PageFileStatic::get_reference()?;
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;

        Ok(TelemetryUpdate {
            static_data: StaticData::from(*static_data),
            physics: Physics::from(*physics_data),
            graphics: Graphics::from(*graphics_data),
        })
    }
}

/// Change Computation
impl Telemetry {
    fn game_state(&mut self, update: &TelemetryUpdate, ctx: &mut <Telemetry as Actor>::Context) {
        if update.graphics.status != self.graphics.status {
            match update.graphics.status {
                telemetry::Status::Off | telemetry::Status::Replay => {
                    if self.connected {
                        self.connected = false;
                        self.router.do_send(super::ShmGameState::Disconnected);
                    }
                }
                telemetry::Status::Live | telemetry::Status::Pause => {
                    if !self.connected {
                        self.connected = true;
                        self.router.do_send(super::ShmGameState::Connected)
                    }
                }
            }
        }
    }

    /// Compute lap results and reset history struct
    fn lap_history(&mut self) {
        self.lap_result.get_avg_min_max(&self.lap_history);
        self.lap_result.number = self.graphics.completed_laps + 1;

        self.router
            .do_send(UiUpdate::LapWheels(std::mem::take(&mut self.lap_result)));

        self.lap_history = LapHistory::default();
    }

    fn update(&mut self, update: TelemetryUpdate) {
        if let Some((_l_physics, l_graphics)) = self.lap_history.last_point() {
            if l_graphics.completed_laps < update.graphics.completed_laps {
                // changed fuel usage per lap
                if l_graphics.fuel_used_per_lap != update.graphics.fuel_used_per_lap {
                    self.router.do_send(setup::SetupChange::FuelPerLap(
                        self.graphics.fuel_used_per_lap,
                    ));
                }

                // Compute lap results and reset history structs
                self.lap_history();
            }
        }

        self.commit_update(update);
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
struct TelemetryUpdate {
    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,
}

impl Actor for Telemetry {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("telemetry started");

        self.interval =
            ctx.run_interval(
                Duration::from_millis(16),
                |telemetry, ctx| match Telemetry::get_update() {
                    Ok(update) => {
                        telemetry.game_state(&update, ctx);
                        telemetry.update(update);
                    }
                    Err(error) => {
                        // This triggers a shmdisconnect to the router, which triggers the disconnect
                        //  for the broadcast API, so to prevent it from running these commands all the
                        //  time we put it behind a gate.
                        // `connected` is also false by default, so this will not run until we connected
                        //  to the game at least once.
                        if telemetry.connected {
                            error!("could not connect to telemetry: {:?}", error);
                            telemetry.router.do_send(super::ShmGameState::Disconnected);
                        };
                    }
                },
            );
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("telemetry stopped");
        ctx.cancel_future(self.interval);
    }
}
