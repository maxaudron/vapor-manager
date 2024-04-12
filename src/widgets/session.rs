use crate::telemetry::{Graphics, Physics, StaticData};

pub fn session_info(ui: &mut egui::Ui, physics: &Physics, graphics: &Graphics, static_data: &StaticData) {
    ui.vertical(|ui| {
        ui.label(format!("Type: {:?}", graphics.session));
        ui.label(format!("Track: {}", static_data.track));
        ui.label(format!("Car: {}", static_data.car_model));

        ui.label(format!("Driver: {} {}", static_data.player_name, static_data.player_surname));
        ui.label(format!("Time Left: {:?}", graphics.session_time_left));

        ui.label(format!("Laps: {:?}", graphics.completed_laps));
        ui.label(format!("Fuel/Lap: {:?}l", graphics.fuel_used_per_lap));

        ui.label(format!("Air Temprature: {:.1?}", physics.air_temperature));
        ui.label(format!("Road Temprature: {:.1?}", physics.road_temperature));
    });
}
