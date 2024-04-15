#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(non_snake_case)]

use dioxus::prelude::*;
use futures_util::stream::StreamExt;

use vapor_manager::{
    components::{
        setups::SetupManager,
        status_bar::StatusBar,
        theme::{Theme, ThemeSwitcher},
        wheels::WheelPressures,
        debug::Debug,
    },
    setup::{SetupChange, SetupManager},
    telemetry::{broadcast::BroadcastState, Telemetry},
    State, StateChange,
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Base)]
        #[route("/")]
        Home {},
        #[route("/settings")]
        Settings {},
        #[route("/debug")]
        Debug {},
}

fn main() {
    let collector = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    const _TAILWIND_URL: &str = manganis::mg!(file("public\\tailwind.css"));

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let setup_state: Signal<Option<SetupManager>> = use_context_provider(|| Signal::new(None));
    let setup_manager: Coroutine<SetupChange> = use_coroutine(|rx| async move {
        to_owned![setup_state];
        SetupManager::coroutine(rx, setup_state).await;
    });

    let state = use_context_provider(|| Signal::new(State::default()));
    let state_manager: Coroutine<StateChange> = use_coroutine(|mut rx| async move {
        to_owned![state];
        while let Some(msg) = rx.next().await {
            match msg {
                StateChange::AvgTyrePressure(wheels) => state.write().avg_tyre_pressures = wheels,
                StateChange::Weather(weather) => state.write().weather = weather,
                StateChange::TrackName(name) => state.write().track_name = name,
                StateChange::SessionType(session) => state.write().session_type = session,
                StateChange::ShmConnected(connected) => state.write().shm_connected = connected,
                StateChange::BroadcastConnected(connected) => {
                    state.write().broadcast_connected = connected
                }
            }
        }
    });

    let state_manager_tx = state_manager.tx();
    let setup_manager_tx = setup_manager.tx();
    use_hook(move || {
        std::thread::spawn(move || {
            let mut telemetry = Telemetry::default();
            telemetry.run(state_manager_tx, setup_manager_tx)
        });
    });

    let state_manager_tx = state_manager.tx();
    let setup_manager_tx = setup_manager.tx();
    use_hook(move || {
        std::thread::spawn(move || {
            let mut broadcast = BroadcastState::new(state_manager_tx, setup_manager_tx);
            broadcast.run()
        });
    });

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Base() -> Element {
    let theme = use_signal(|| Theme::Mocha);
    let route = use_route::<Route>();

    let theme_lower = format!("{theme:?}").to_lowercase();

    rsx! {
        div {
            class: "h-[100vh] w-[100vw] grid grid-rows-[auto_minmax(0,_1fr)_auto] {theme_lower} bg-crust",
            "data-theme": "{theme:?}",
            div { class: "grid grid-cols-2",
                div { class: "justify-self-start",
                    ul { class: "menu menu-horizontal rounded-box gap-2 bg-sla",
                        li {
                            Link {
                                class: if (route == Route::Home {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Home {},
                                "Home"
                            }
                        }
                        li {
                            Link {
                                class: if (route == Route::Settings {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Settings {},
                                "Settings"
                            }
                        }
                        li {
                            Link {
                                class: if (route == Route::Debug {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Debug {},
                                "Debug"
                            }
                        }
                    }
                }
                div { class: "justify-self-end",
                    ThemeSwitcher { theme }
                }
            }
            Outlet::<Route> {}
            StatusBar {}
        }
    }
}

#[component]
fn Settings() -> Element {
    rsx! { "Blog post" }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "grid grid-rows-[min-content_auto] gap-2 px-2",
            div { class: "grid grid-cols-[min-content_auto] gap-2 h-min",
                WheelPressures {}
                SetupManager {}
            }
            div { class: "bg-base rounded-lg shadow-lg h-auto", "aaaa" }
        }
    }
}
