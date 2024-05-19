mod conversion;
mod data;
mod laphistory;
pub use laphistory::*;
pub mod shm;

pub mod broadcast;

use std::{fmt::Display, time::Duration};

pub use data::*;
use dioxus::hooks::UnboundedSender;
pub use shm::*;
use thiserror::Error;
use tracing::{debug, error};

use crate::{setup::SetupChange, StateChange};

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub connected: bool,
    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,
    pub lap_result: Lap,
    pub lap_history: LapHistory,
    pub laps: Vec<Lap>,

    pub state_tx: UnboundedSender<StateChange>,
    pub setup_tx: UnboundedSender<SetupChange>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LapTime(Duration);

impl From<Duration> for LapTime {
    fn from(value: Duration) -> Self {
        LapTime(value)
    }
}

impl Display for LapTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let minutes = secs / 60;
        let secs = secs - (minutes * 60);
        let millis = self.0.subsec_millis();
        write!(f, "{:02}:{:02}.{:03}", minutes, secs, millis)
    }
}

impl LapTime {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Lap {
    pub number: i32,
    pub sectors: Vec<LapTime>,
    pub time: LapTime,
    pub valid: bool,
    pub tyre_pressure: AvgMinMax<Wheels<f32>>,
    pub tyre_temperature: AvgMinMax<Wheels<f32>>,
    pub brake_temperature: AvgMinMax<Wheels<f32>>,
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
            lap_result: Default::default(),
            lap_history: Default::default(),
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
        self.lap_history = LapHistory::default();
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

                            self.setup_tx
                                .unbounded_send(SetupChange::FuelPerLap(
                                    self.graphics.fuel_used_per_lap,
                                ))
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
                            if let Some((_l_physics, l_graphics)) = self.lap_history.last_point() {
                                if l_graphics.current_sector_index
                                    < self.graphics.current_sector_index
                                {
                                    self.lap_result.sectors.push(
                                        Duration::from_millis(
                                            self.graphics.lap_timing.last_sector_ms as u64,
                                        )
                                        .into(),
                                    );
                                }

                                if l_graphics.completed_laps < self.graphics.completed_laps
                                {
                                    // For future development with more detailed metrics
                                    // self.laps.push(self.current_lap.clone());

                                    if l_graphics.fuel_used_per_lap
                                        != self.graphics.fuel_used_per_lap
                                    {
                                        self.setup_tx
                                            .unbounded_send(SetupChange::FuelPerLap(
                                                self.graphics.fuel_used_per_lap,
                                            ))
                                            .unwrap();
                                    }

                                    self.lap_result.sectors.push(
                                        Duration::from_millis(
                                            self.graphics.lap_timing.last.millis as u64,
                                        )
                                        .into(),
                                    );
                                    self.lap_result.sectors = self
                                        .lap_result
                                        .sectors
                                        .iter()
                                        .scan(0u128, |state, sector| {
                                            let new = Duration::from_millis(
                                                (sector.0.as_millis() - *state) as u64,
                                            );
                                            *state = sector.0.as_millis();
                                            Some(new.into())
                                        })
                                        .collect();

                                    self.lap_result.time = Duration::from_millis(
                                        self.graphics.lap_timing.last.millis as u64,
                                    )
                                    .into();

                                    self.lap_result.get_avg_min_max(&self.lap_history);
                                    self.lap_result.number = self.graphics.completed_laps;
                                    self.lap_result.valid = l_graphics.is_valid_lap;

                                    debug!("finished lap: {:?}", self.lap_result);

                                    self.state_tx
                                        .unbounded_send(StateChange::Lap(std::mem::take(
                                            &mut self.lap_result,
                                        )))
                                        .unwrap();

                                    self.lap_history = LapHistory::default();
                                }
                            }

                            if p_changed {
                                self.lap_history.h_physics.push(self.physics.clone());
                            }

                            if g_changed {
                                self.lap_history.h_graphics.push(self.graphics.clone());
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
    #[cfg(windows)]
    #[error("failed to connect to acc: {0}")]
    ConnectionFailed(windows::core::Error),
    #[error("acc offline")]
    Offline,
}
