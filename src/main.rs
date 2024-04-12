#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use acc_tools::ACCTools;

fn main() -> eframe::Result<()> {
    let subscriber = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
            // .with_icon(
            //     // NOTE: Adding an icon is optional
            //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
            //         .unwrap(),
            // ),
        ..Default::default()
    };
    eframe::run_native(
        "ACC Tools",
        native_options,
        Box::new(|cc| Box::new(ACCTools::new(cc))),
    )
}
