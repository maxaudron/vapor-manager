#![warn(clippy::all, rust_2018_idioms)]

mod app;
use std::sync::{Arc, RwLock};

pub use app::ACCTools;
use setup::SetupManager;
use telemetry::{Status, Telemetry};
use tracing::{debug, error};

pub mod setup;
pub mod telemetry;

mod widgets;

#[derive(Debug, Default, Clone)]
pub struct State {
    setup_manager: SetupManager,
    telemetry: Telemetry,
}

impl State {
    pub fn run(state: Arc<RwLock<State>>, ctx: &egui::Context) {
        loop {
            {
                let mut state = state.write().unwrap();
                match state.telemetry.connect() {
                    Ok(()) => (),
                    Err(e) => {
                        error!("failed to connect to acc: {e}");
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        continue;
                    }
                }
            }

            loop {
                std::thread::sleep(std::time::Duration::from_millis(16));

                let mut state = state.write().unwrap();
                match state.telemetry.refresh() {
                    Ok(changed) => {
                        if changed {
                            if state.telemetry.graphics.status == Status::Live {
                                if state.setup_manager.setups.is_empty() {
                                    match SetupManager::discover(
                                        &state.telemetry.static_data.car_model,
                                        &state.telemetry.static_data.track,
                                    ) {
                                        Ok(s) => state.setup_manager = s,
                                        Err(_) => {
                                            error!("failed to load setups");
                                        }
                                    }
                                }

                                if state.setup_manager.target_temperature
                                    != state.telemetry.physics.air_temperature.round() as i32
                                {
                                    let air =
                                        state.telemetry.physics.air_temperature.round() as i32;
                                    let road =
                                        state.telemetry.physics.road_temperature.round() as i32;

                                    state.setup_manager.adjust_pressure(air, road);
                                    state.setup_manager.store();
                                }
                            }

                            ctx.request_repaint();
                        }
                    }
                    Err(e) => {
                        error!("failed to refresh acc data: {e}");
                        state.telemetry.connected = false;
                        break;
                    }
                }
            }
        }
    }
}
