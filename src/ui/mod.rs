use components::{Base, Home, Settings, Setups, Theme};
use dioxus::{
    desktop::{tao::window::Icon, Config, LogicalSize, WindowBuilder},
    prelude::*,
};

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
        Settings {},
        // #[route("/debug")]
        // #[cfg(debug_assertions)]
        // Debug {},
}

pub fn launch(router: actix::Addr<crate::actors::Router>) -> () {
    const _TAILWIND_URL: &str = manganis::mg!(file("public\\tailwind.css"));

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
    let theme = use_context_provider(|| Signal::new(Theme::Mocha));
    let settings: Signal<Settings> = use_context_provider(|| Signal::new(Settings::init(theme)));

    rsx! {
        Router::<Route> {}
    }
}
