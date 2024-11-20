use std::time::Duration;

use actix::prelude::*;
use tracing::debug;

use crate::{
    setup,
    telemetry::{
        self, Graphics, LapHistory, PageFileGraphics, PageFilePhysics, PageFileStatic, Physics,
        SharedMemoryPage, StaticData, TelemetryError,
    },
    StateChange,
};

use super::Router;

pub struct Telemetry {
    interval: SpawnHandle,
    router: Addr<Router>,

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
            static_data: Default::default(),
            physics: Default::default(),
            graphics: Default::default(),
            lap_history: Default::default(),
            lap_result: Default::default(),
        }
    }

    /// Read initial values from memory shim
    fn connect(&mut self) -> Result<bool, TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;
        let static_data = PageFileStatic::get_reference()?;

        self.static_data = StaticData::from(*static_data);
        self.physics = Physics::from(*physics_data);
        self.graphics = Graphics::from(*graphics_data);

        Ok(graphics_data.status.data != 0)
    }

    /// Commit the update to data to be used in the next round
    fn commit_update(&mut self, update: TelemetryUpdate) {
        self.static_data = update.static_data;

        if update.graphics.packet_id > self.graphics.packet_id {
            self.graphics = update.graphics;
            self.lap_history.h_graphics.push(self.graphics.clone());
        }

        // TODO this might be causing the problems with the repeated lap times
        // what happens when this number reaches it's max?
        if update.physics.packet_id > self.physics.packet_id {
            self.physics = update.physics;
            self.lap_history.h_physics.push(self.physics.clone());
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
    fn game_state(&self, update: &TelemetryUpdate) {
        if update.graphics.status != self.graphics.status {
            match update.graphics.status {
                telemetry::Status::Off | telemetry::Status::Replay => {
                    self.router.do_send(super::ShmGameState::Disconnected)
                }
                telemetry::Status::Live | telemetry::Status::Pause => {
                    self.router.do_send(super::ShmGameState::Connected)
                }
            }
        }
    }

    /// Compute lap results and reset history struct
    fn lap_history(&mut self) {
        self.lap_result.get_avg_min_max(&self.lap_history);
        self.lap_result.number = self.graphics.completed_laps;

        debug!("finished lap: {:?}", self.lap_result);

        self.router
            .do_send(StateChange::LapWheels(std::mem::take(&mut self.lap_result)));

        self.lap_history = LapHistory::default();
    }

    fn update(&mut self, update: TelemetryUpdate) {
        self.game_state(&update);

        if let Some((_l_physics, l_graphics)) = self.lap_history.last_point() {
            if l_graphics.completed_laps < self.graphics.completed_laps {
                // changed fuel usage per lap
                if l_graphics.fuel_used_per_lap != self.graphics.fuel_used_per_lap {
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
        match self.connect() {
            Ok(_) => (),
            Err(_) => ctx.terminate(),
        }

        self.interval = ctx.run_interval(Duration::from_millis(16), |telemetry, ctx| {
            debug!("telemetry tick");
            match Telemetry::get_update() {
                Ok(update) => telemetry.update(update),
                Err(_) => ctx.terminate(),
            }
        });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("telemetry stopped");
        ctx.cancel_future(self.interval);
    }
}
