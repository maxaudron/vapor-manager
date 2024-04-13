use crate::setup::SetupManager;

pub fn setups(ui: &mut egui::Ui, setup_manager: &SetupManager) {
    ui.vertical(|ui| {
        ui.label(format!("Loaded Setups for {}:", setup_manager.track));
        for setup in &setup_manager.setups {
            ui.label(format!("{} ({} {})", setup.name, setup.air_temperature, setup.road_temperature));
        };
        
        ui.label("Adjusted Setups:");
        for setup in &setup_manager.adj_setups {
            ui.label(format!("{} ({} {})", setup.name, setup.air_temperature, setup.road_temperature));
        };
    });
}
