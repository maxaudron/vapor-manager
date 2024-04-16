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

use self::broadcast::BroadcastMsg;

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub connected: bool,
    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,
    pub current_lap: Lap,
    pub laps: Vec<Lap>,

    pub state_tx: UnboundedSender<StateChange>,
    pub setup_tx: UnboundedSender<SetupChange>,
}

#[derive(Default, Debug, Clone)]
pub struct Lap {
    pub h_physics: Vec<Physics>,
    pub h_graphics: Vec<Graphics>,
}

impl Lap {
    pub fn last_point(&self) -> Option<(&Physics, &Graphics)> {
        if self.h_physics.last().is_some() && self.h_graphics.last().is_some() {
            Some((
                self.h_physics.last().unwrap(),
                self.h_graphics.last().unwrap(),
            ))
        } else {
            None
        }
    }
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
    pub fn new(
        state_tx: UnboundedSender<StateChange>,
        setup_tx: UnboundedSender<SetupChange>,
    ) -> Telemetry {
        Telemetry {
            connected: false,
            state_tx,
            setup_tx,
            static_data: Default::default(),
            physics: Default::default(),
            graphics: Default::default(),
            current_lap: Default::default(),
            laps: Default::default(),
        }
    }

    pub fn connect(&mut self) -> Result<bool, TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;
        let static_data = PageFileStatic::get_reference()?;

        self.static_data = StaticData::from(*static_data);
        self.physics = Physics::from(*physics_data);
        self.graphics = Graphics::from(*graphics_data);

        Ok(graphics_data.status.data != 0)
    }

    pub fn refresh(&mut self) -> Result<(bool, bool), TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;

        let (mut p_changed, mut g_changed) = (false, false);

        if graphics_data.packet_id > self.graphics.packet_id {
            self.graphics = Graphics::from(*graphics_data);
            g_changed = true;
        }

        if physics_data.packet_id > self.physics.packet_id {
            self.physics = Physics::from(*physics_data);
            p_changed = true;
        }

        Ok((p_changed, g_changed))
    }

    pub fn send_connected(&mut self) {
        if !self.connected {
            self.state_tx
                .unbounded_send(StateChange::ShmConnected(true))
                .unwrap();

            self.connected = true;
        }
    }

    pub fn send_disconnected(&mut self) {
        if self.connected {
            self.state_tx
                .unbounded_send(StateChange::ShmConnected(false))
                .unwrap();

            self.connected = false;
        }
    }

    pub fn cleanup(&mut self) {
        self.laps = Vec::new();
        self.current_lap = Lap::default();
    }

    pub fn run(&mut self) {
        debug!("started acc shm event loop");
        loop {
            {
                // Connection loop loads Static Data as well as first Physics and Graphics
                // Static Data is only loaded on connect, thus it is important to reenter this
                // connection loop if the games goes into Status::Off as the map might change
                // and we need to reload the static data.
                match self.connect() {
                    Ok(connected) => {
                        if connected {
                            debug!("connected to acc");

                            self.send_connected();
                            self.setup_tx
                                .unbounded_send(SetupChange::Load((
                                    self.static_data.car_model.to_string(),
                                    self.static_data.track.to_string(),
                                )))
                                .unwrap();
                        } else {
                            debug!("acc is offline, waiting for session");
                            std::thread::sleep(Duration::from_millis(500));
                            continue;
                        }
                    }
                    Err(e) => {
                        error!("failed to connect to shared memory: {:?}", e);
                        self.send_disconnected();
                        std::thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                }
            }

            // Game Telemetry Loop
            // Refreshes Graphics and Physics page at a high rate to gather game telemetry
            loop {
                match self.refresh() {
                    Ok((p_changed, g_changed)) => {
                        if self.graphics.status == Status::Live {
                            if let Some((_l_physics, l_graphics)) = self.current_lap.last_point() {
                                if l_graphics.fuel_used_per_lap != self.graphics.fuel_used_per_lap
                                    || l_graphics.lap_timing.best != self.graphics.lap_timing.best
                                {
                                    self.setup_tx
                                        .unbounded_send(SetupChange::LapInfo((
                                            self.graphics.fuel_used_per_lap,
                                            self.graphics.lap_timing.best.clone(),
                                        )))
                                        .unwrap();
                                }

                                if l_graphics.completed_laps < self.graphics.completed_laps {
                                    self.laps.push(self.current_lap.clone());

                                    debug!("finished lap: {:?}", self.graphics.completed_laps);
                                    self.state_tx
                                        .unbounded_send(StateChange::AvgTyrePressure(
                                            self.current_lap.avg_tyre_pressures(),
                                        ))
                                        .unwrap();
                                    self.current_lap = Lap::default();
                                }
                            }

                            if p_changed {
                                self.current_lap.h_physics.push(self.physics.clone());
                            }

                            if g_changed {
                                self.current_lap.h_graphics.push(self.graphics.clone());
                            }
                        } else if self.graphics.status == Status::Off {
                            if self.connected {
                                debug!("acc is offline, switching to connection loop");
                                self.send_disconnected();
                                self.cleanup();
                            }

                            break;
                        }

                        std::thread::sleep(Duration::from_millis(16));
                    }
                    Err(e) => {
                        error!("failed to retrive data: {:?}", e);
                        self.send_disconnected();
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
