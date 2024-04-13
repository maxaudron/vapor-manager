use std::sync::{Arc, RwLock};

// use simetry::assetto_corsa_competizione::Client as TelemetryClient;

use egui::Color32;
use tracing::{debug, error};

use crate::{
    setup::{Setup, SetupManager},
    telemetry::{self, Status, Telemetry},
    State,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ACCTools {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    state: Arc<RwLock<State>>,
}

impl ACCTools {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, state: Arc<RwLock<State>>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut acctools = eframe::get_value(storage, eframe::APP_KEY).unwrap_or(Self {
                label: "Hello World!".to_owned(),
                value: 2.7,
                state: state.clone(),
            });

            acctools.state = state;
            return acctools;
        }

        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            state,
        }
    }
}

impl eframe::App for ACCTools {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = self.state.read().unwrap();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.style_mut().debug.debug_on_hover = true;
            ui.visuals_mut().override_text_color = Some(Color32::WHITE);

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().debug.debug_on_hover = true;

            if state.telemetry.graphics.status == Status::Off {
                egui::Frame::default()
                    .fill(Color32::DARK_RED)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Assetto Corsa Competizione not running")
                                .font(egui::FontId::proportional(24.0)),
                        );
                    });
            } else {
                ui.horizontal(|ui| {
                    crate::widgets::tyre_pressure(ui, &state.telemetry.physics, self.value);
                    crate::widgets::session_info(
                        ui,
                        &state.telemetry.physics,
                        &state.telemetry.graphics,
                        &state.telemetry.static_data,
                    )
                });

                crate::widgets::setups(ui, &state.setup_manager);
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
