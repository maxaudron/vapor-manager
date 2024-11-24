use components::{Base, Home, Settings, SettingsComponent, Setups};
use dioxus::{
    desktop::{tao::window::Icon, Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use document::Stylesheet;

use crate::PROGRAM_NAME;

mod components;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Base)]
        #[route("/")]
        Home {},
        #[route("/setups")]
        Setups {},
        #[route("/settings")]
        SettingsComponent {},
        // #[route("/debug")]
        // #[cfg(debug_assertions)]
        // Debug {},
}

pub fn launch(router: actix::Addr<crate::actors::Router>) -> () {
    let mut config = Config::new().with_disable_context_menu(true);

    #[cfg(windows)]
    {
        let mut documents =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        documents.push(PROGRAM_NAME);
        documents.push("webview");
        config = config.with_data_directory(documents)
    }

    let bin: &[u8] = include_bytes!("../../icons/icon.bin");
    let rgba = Icon::from_rgba(bin.to_owned(), 460, 460).expect("image parse failed");

    #[cfg(not(debug_assertions))]
    let config = config.with_menu(None);
    let size = LogicalSize::new(1250, 800);

    LaunchBuilder::desktop()
        .with_cfg(
            config
                .with_window(
                    WindowBuilder::new()
                        .with_resizable(true)
                        .with_inner_size(size)
                        .with_min_inner_size(size)
                        .with_title(PROGRAM_NAME),
                )
                .with_icon(rgba),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    //
    // Settings
    let settings: Signal<Settings> = use_context_provider(|| Signal::new(Settings::init()));

    const TAILWIND_URL: Asset = asset!("/public/tailwind.css");

    rsx! {
        Stylesheet { href: TAILWIND_URL }
        Router::<Route> {}
    }
}
