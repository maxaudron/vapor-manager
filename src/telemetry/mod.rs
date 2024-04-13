mod conversion;
mod data;
pub mod shm;

mod broadcast;

pub use data::*;
pub use shm::*;
use thiserror::Error;
use tracing::{debug, error, trace};

#[derive(Default, Debug, Clone)]
pub struct Telemetry {
    pub connected: bool,
    pub static_data: StaticData,
    pub physics: Physics,
    pub graphics: Graphics,
}

impl Telemetry {
    pub fn new() -> Telemetry {
        Telemetry {
            connected: false,
            ..Default::default()
        }
    }
    pub fn connect(&mut self) -> Result<(), TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;
        let static_data = PageFileStatic::get_reference()?;
        debug!("got initial data: {:?}", static_data);
        debug!("got initial data: {:?}", physics_data);
        debug!("got initial data: {:?}", graphics_data);

        *self = Self {
            connected: graphics_data.status.data != 0,
            static_data: StaticData::from(*static_data),
            physics: Physics::from(*physics_data),
            graphics: Graphics::from(*graphics_data),
        };

        Ok(())
    }

    pub fn refresh(&mut self) -> Result<bool, TelemetryError> {
        let graphics_data = PageFileGraphics::get_reference()?;
        let physics_data = PageFilePhysics::get_reference()?;

        let mut changed = false;

        if graphics_data.packet_id > self.graphics.packet_id {
            // trace!("updated graphics data");
            self.connected = graphics_data.status.data != 0;
            self.graphics = Graphics::from(*graphics_data);
            changed = true;
        }

        if physics_data.packet_id > self.physics.packet_id {
            // trace!("updated physics data");
            self.physics = Physics::from(*physics_data);
            changed = true;
        }

        Ok(changed)
    }
}

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("failed to connect to acc")]
    ConnectionFailed,
    #[error("acc offline")]
    Offline,
}
