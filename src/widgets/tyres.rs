use egui::{epaint::Hsva, vec2, Color32, NumExt, Rect, RichText, Rounding, Sense, Stroke, Widget};

use crate::telemetry::Physics;

/// Tyre Pressure Gauges
///
/// Optimum Pressure is 26 - 27.2 PSI
pub fn tyre_pressure(ui: &mut egui::Ui, physics: &Physics, v: f32) {
    let fl = physics.wheels.front_left.tyre_pressure;
    let fr = physics.wheels.front_right.tyre_pressure;
    let rl = physics.wheels.rear_left.tyre_pressure;
    let rr = physics.wheels.rear_right.tyre_pressure;

    let text = |t| RichText::new(t).font(egui::FontId::proportional(18.0));

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(text(format!("{:.1}", fl)));
            pressure_bar(ui, pressure_optimum_percentage(fl));
            ui.label(RichText::new("FL").font(egui::FontId::proportional(18.0)));
        });
        ui.vertical(|ui| {
            ui.label(text(format!("{:.1}", fr)));
            pressure_bar(ui, pressure_optimum_percentage(fr));
            ui.label(RichText::new("FR").font(egui::FontId::proportional(18.0)));
        });
        ui.vertical(|ui| {
            ui.label(text(format!("{:.1}", rl)));
            pressure_bar(ui, pressure_optimum_percentage(rl));
            ui.label(RichText::new("RL").font(egui::FontId::proportional(18.0)));
        });
        ui.vertical(|ui| {
            ui.label(text(format!("{:.1}", rr)));
            pressure_bar(ui, pressure_optimum_percentage(rr));
            ui.label(RichText::new("RR").font(egui::FontId::proportional(18.0)));
        });
    });
}

/// Tire Pressures in percentage with 50% being the optimum
/// minimum 23
/// maximum 30
fn pressure_optimum_percentage(pressure: f32) -> f32 {
    (pressure - 23.0) / (30.0 - 23.0)
}

fn pressure_color(percentage: f32) -> Color32 {
    Hsva::new(0.66 * (1.0 - percentage), 1.0, 0.64, 1.0).into()
}

// hsl(0 100% 64%)
// hsl(240 100% 64%)

fn pressure_bar(ui: &mut egui::Ui, progress: f32) {
    let desired_width = ui.spacing().interact_size.y;
    let height = 196.0;
    let (outer_rect, response) =
        ui.allocate_exact_size(vec2(desired_width, height), Sense::hover());

    if ui.is_rect_visible(response.rect) {
        let corner_radius = outer_rect.width() / 2.0;
        let rounding: Rounding = corner_radius.into();
        ui.painter().rect(
            outer_rect,
            rounding,
            ui.visuals().extreme_bg_color,
            Stroke::NONE,
        );
        let min_height = 2.0 * rounding.sw.at_least(rounding.nw).at_most(corner_radius);
        let filled_height = (outer_rect.height() * progress).at_least(min_height);
        let mut start = outer_rect.min;
        start.y = start.y + (outer_rect.height() - filled_height);
        let inner_rect = Rect::from_min_size(start, vec2(outer_rect.width(), filled_height));

        ui.painter()
            .rect(inner_rect, rounding, pressure_color(progress), Stroke::NONE);
    };
}
