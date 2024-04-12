use std::sync::Arc;

// use simetry::assetto_corsa_competizione::Client as TelemetryClient;

use egui::Color32;

use crate::{
    setup::Setup,
    telemetry::{self, Telemetry},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ACCTools {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    setup: Option<Setup>,
    #[serde(skip)]
    telemetry: Option<Telemetry>,
}

impl Default for ACCTools {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            setup: None,
            telemetry: None,
        }
    }
}

impl ACCTools {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ACCTools {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if let Some(telemetry) = self.telemetry.as_mut() {
            let _ = telemetry.refresh();
        } else {
            match Telemetry::connect() {
                Ok(telemetry) => self.telemetry = Some(telemetry),
                Err(_) => (),
            }
        }

        let top_panel_bg = if self.telemetry.is_none() {
            Color32::DARK_RED
        } else {
            Color32::BLACK
        };

        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::default().outer_margin(0.0).fill(top_panel_bg))
            .show(ctx, |ui| {
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

                if self.telemetry.is_none() {
                    ui.vertical_centered_justified(|ui| {
                        ui.label(
                            egui::RichText::new("No connection to Assetto Corsa Competizione")
                                .font(egui::FontId::proportional(24.0)),
                        )
                    });
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().debug.debug_on_hover = true;

            if self.telemetry.is_none() {
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
                    crate::widgets::tyre_pressure(
                        ui,
                        &self.telemetry.as_ref().unwrap().physics,
                        self.value,
                    );
                    let telemetry = self.telemetry.as_ref().unwrap();
                    crate::widgets::session_info(
                        ui,
                        &telemetry.physics,
                        &telemetry.graphics,
                        &telemetry.static_data,
                    )
                });
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
