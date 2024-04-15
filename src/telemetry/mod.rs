mod conversion;
mod data;
pub mod shm;

pub mod broadcast;

use std::time::Duration;

pub use data::*;
use dioxus::hooks::UnboundedSender;
pub use shm::*;
use thiserror::Error;
use tracing::{debug, error};

use crate::{setup::SetupChange, StateChange};

#[derive(Default, Debug, Clone)]
pub struct Telemetry {
    pub connected: bool,
    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,
    pub current_lap: Lap,
    pub laps: Vec<Lap>,
}

#[derive(Default, Debug, Clone)]
pub struct Lap {
    pub h_physics: Vec<Physics>,
    pub h_graphics: Vec<Graphics>,
}

impl Lap {
    pub fn avg_tyre_pressures(&self) -> Wheels<f32> {
        let (fl, fr, rl, rr) = self
            .h_physics
            .iter()
            .fold((0.0, 0.0, 0.0, 0.0), |mut wheels, p| {
                wheels.0 += p.wheels.front_left.tyre_pressure;
                wheels.1 += p.wheels.front_right.tyre_pressure;
                wheels.2 += p.wheels.rear_left.tyre_pressure;
                wheels.3 += p.wheels.rear_right.tyre_pressure;

                wheels
            });

        let count = self.h_physics.len();

        Wheels {
            front_left: fl / count as f32,
            front_right: fr / count as f32,
            rear_left: rl / count as f32,
            rear_right: rr / count as f32,
        }
    }
}

impl Telemetry {
    pub fn new() -> Telemetry {
        Telemetry {
            connected: false,
            ..Default::default()
        }
    }

    pub fn connect(&mut self) -> Result<bool, TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;
        let static_data = PageFileStatic::get_reference()?;

        *self = Self {
            connected: graphics_data.status.data != 0,
            static_data: StaticData::from(*static_data),
            physics: Physics::from(*physics_data),
            graphics: Graphics::from(*graphics_data),
            current_lap: Lap::default(),
            laps: Vec::new(),
        };

        Ok(graphics_data.status.data != 0)
    }

    pub fn refresh(&mut self) -> Result<(bool, bool), TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;

        let (mut p_changed, mut g_changed) = (false, false);

        if graphics_data.packet_id > self.graphics.packet_id {
            self.connected = graphics_data.status.data != 0;
            self.graphics = Graphics::from(*graphics_data);
            g_changed = true;
        }

        if physics_data.packet_id > self.physics.packet_id {
            self.physics = Physics::from(*physics_data);
            p_changed = true;
        }

        Ok((p_changed, g_changed))
    }

    pub fn run(
        &mut self,
        state_tx: UnboundedSender<StateChange>,
        setup_tx: UnboundedSender<SetupChange>,
    ) {
        debug!("started acc shm event loop");
        loop {
            {
                match self.connect() {
                    Ok(connected) => {
                        if connected {
                            debug!("connected to acc");

                            state_tx
                                .unbounded_send(StateChange::ShmConnected(true))
                                .unwrap();

                            setup_tx
                                .unbounded_send(SetupChange::Load((
                                    self.static_data.car_model.to_string(),
                                    self.static_data.track.to_string(),
                                )))
                                .unwrap();
                        } else {
                            debug!("acc is offline, waiting for session");
                            state_tx
                                .unbounded_send(StateChange::ShmConnected(false))
                                .unwrap();
                            std::thread::sleep(Duration::from_millis(500));
                            continue;
                        }
                    }
                    Err(e) => {
                        error!("failed to connect to shared memory: {:?}", e);
                        state_tx
                            .unbounded_send(StateChange::ShmConnected(false))
                            .unwrap();
                        std::thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                }
            }

            loop {
                match self.refresh() {
                    Ok((p_changed, g_changed)) => {
                        if self.graphics.status == Status::Live {
                            if self
                                .current_lap
                                .h_graphics
                                .last()
                                .unwrap_or(&Graphics::default())
                                .completed_laps
                                < self.graphics.completed_laps
                            {
                                self.laps.push(self.current_lap.clone());

                                debug!("finished lap: {:?}", self.graphics.completed_laps);
                                state_tx
                                    .unbounded_send(StateChange::AvgTyrePressure(
                                        self.current_lap.avg_tyre_pressures(),
                                    ))
                                    .unwrap();
                                self.current_lap = Lap::default();
                            }

                            if p_changed {
                                self.current_lap.h_physics.push(self.physics.clone());
                            }

                            if g_changed {
                                self.current_lap.h_graphics.push(self.graphics.clone());
                            }
                        } else if self.graphics.status == Status::Off {
                            debug!("acc is offline, switching to connection loop");
                            self.connected = false;
                            state_tx
                                .unbounded_send(StateChange::ShmConnected(false))
                                .unwrap();
                            std::thread::sleep(Duration::from_millis(500));
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(16));
                    }
                    Err(e) => {
                        error!("failed to retrive data: {:?}", e);
                        self.connected = false;
                        state_tx
                            .unbounded_send(StateChange::ShmConnected(false))
                            .unwrap();

                        std::thread::sleep(Duration::from_millis(500));
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("failed to connect to acc: {0}")]
    ConnectionFailed(windows::core::Error),
    #[error("acc offline")]
    Offline,
}
