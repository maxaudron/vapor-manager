pub mod shm;
mod data;
mod conversion;

use std::sync::{atomic::AtomicBool, Arc};

pub use shm::*;
pub use data::*;
use thiserror::Error;

pub struct Telemetry {
    pub connected: bool,
    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,
}

impl Telemetry {
    pub fn connect() -> Result<Self, TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;
        let static_data = PageFileStatic::get_reference()?;

        Ok(Self {
            connected: graphics_data.status.data != 0,
            static_data: StaticData::from(*static_data),
            physics: Physics::from(*physics_data),
            graphics: Graphics::from(*graphics_data),
        })
    }

    pub fn refresh(&mut self) -> Result<(), TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;

        if graphics_data.packet_id > self.graphics.packet_id {
            self.graphics = Graphics::from(*graphics_data)
        }

        if physics_data.packet_id > self.physics.packet_id {
            self.physics = Physics::from(*physics_data)
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("failed to connect to acc")]
    ConnectionFailed,
    #[error("acc offline")]
    Offline
}
