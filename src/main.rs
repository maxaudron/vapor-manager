#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(non_snake_case)]

use dioxus::prelude::*;
use futures_util::stream::StreamExt;

use vapor_manager::{
    components::{
        status_bar::StatusBar,
        theme::{Theme, ThemeSwitcher},
        wheels::WheelPressures,
    },
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
}

fn main() {
    tracing_subscriber::fmt::init();

    const _TAILWIND_URL: &str = manganis::mg!(file("public\\tailwind.css"));

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let state = use_context_provider(|| Signal::new(State::default()));

    let state_manager: Coroutine<StateChange> = use_coroutine(|mut rx| async move {
        to_owned![state];
        while let Some(msg) = rx.next().await {
            match msg {
                StateChange::AvgTyrePressure(wheels) => state.write().avg_tyre_pressures = wheels,
                StateChange::Weather(weather) => state.write().weather = weather,
                StateChange::TrackName(name) => state.write().track_name = name,
                StateChange::SessionType(session) => state.write().session_type = session,
            }
        }
    });

    let state_manager_tx = state_manager.tx();
    use_hook(move || {
        std::thread::spawn(move || {
            let mut telemetry = Telemetry::default();
            telemetry.run(state_manager_tx)
        });
    });

    let state_manager_tx = state_manager.tx();
    use_hook(move || {
        std::thread::spawn(move || {
            let mut broadcast = BroadcastState::new(state_manager_tx);
            broadcast.run()
        });
    });

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Base() -> Element {
    let theme = use_signal(|| Theme::Dark);
    let route = use_route::<Route>();

    rsx! {
        div { class: "h-[100vh] w-[100vw] grid grid-rows-[auto_minmax(0,_1fr)_auto]", "data-theme": "{theme:?}",
            div { class: "grid grid-cols-2",
                div { class: "justify-self-start",
                    ul { class: "menu menu-horizontal bg-base-100 rounded-box gap-2",
                        li { Link { class: if (route == Route::Home {}) { "btn btn-active-primary" } else { "btn" },
                            to: Route::Home {}, "Home"
                        }}
                        li { Link { class: if (route == Route::Settings {}) { "btn btn-active-primary" } else { "btn" },
                            to: Route::Settings {}, "Settings"
                        }}
                    }
                }
                div { class: "justify-self-end",
                    ThemeSwitcher { theme: theme }
                }
            }
            Outlet::<Route> {}
            StatusBar {}
        }
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        "Blog post"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "grid auto-cols-min px-2",
            WheelPressures {}
        }
    }
}
