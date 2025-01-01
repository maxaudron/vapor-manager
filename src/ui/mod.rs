use components::{Base, Home, Settings, SettingsComponent, Setups};
use dioxus::{
    desktop::{tao::window::Icon, Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use document::Stylesheet;

use crate::{
    actors::{
        fuel_calculator::{FuelData, FuelMessage},
        ui::{SessionInfo, UiState},
        ClientManagement,
    },
    PROGRAM_NAME,
};

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

pub fn launch() -> () {
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

#[allow(unused)]
type Backend = actix::Addr<crate::actors::Router>;

#[component]
fn App() -> Element {
    //
    // Settings
    let settings: Signal<Settings> = use_context_provider(|| Signal::new(Settings::init()));

    // Initialize blank session states
    let track_info: SyncSignal<SessionInfo> =
        use_context_provider(|| SyncSignal::new_maybe_sync(SessionInfo::default()));
    let laps: SyncSignal<crate::actors::ui::Laps> =
        use_context_provider(|| SyncSignal::new_maybe_sync(crate::actors::ui::Laps::default()));
    let setups: SyncSignal<crate::actors::ui::Setups> =
        use_context_provider(|| SyncSignal::new_maybe_sync(crate::actors::ui::Setups::default()));
    let fuel_data: SyncSignal<FuelData> =
        use_context_provider(|| SyncSignal::new_maybe_sync(FuelData::default()));

    // Initialize Main Arbiter & Background processes
    let arbiter = actix::Arbiter::new();
    let router = crate::actors::Router::initialize(arbiter.handle());
    let router = use_context_provider(|| router);

    router.do_send(FuelMessage::ReserveLaps(settings.read().reserve_laps));

    // Initialize Main UI State and add client to backend
    let ui_state = UiState::initialize(
        router.clone(),
        track_info.clone(),
        laps.clone(),
        setups.clone(),
        fuel_data.clone(),
    );
    router.do_send(ClientManagement::Add(ui_state.clone()));
    let _ = use_context_provider(|| ui_state);

    rsx! {
        Stylesheet { href: asset!("/public/tailwind.css") }
        Router::<Route> {}
    }
}
