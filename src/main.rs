#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(non_snake_case)]

use dioxus::prelude::*;
use futures_util::stream::StreamExt;

use telemetry::{SessionType, Wheels};

pub mod setup;
pub mod telemetry;

// mod widgets;
pub mod components;

use components::{
    base::{Base, Home, Settings},
    debug::Debug,
};

use crate::{
    setup::{SetupChange, SetupManager},
    telemetry::{broadcast::BroadcastState, Telemetry},
};

#[derive(Debug, Clone, PartialEq)]
pub enum StateChange {
    AvgTyrePressure(Wheels<f32>),
    Weather(Weather),
    TrackName(String),
    SessionType(SessionType),
    ShmConnected(bool),
    BroadcastConnected(bool),
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub debug: bool,
    pub shm_connected: bool,
    pub broadcast_connected: bool,
    pub avg_tyre_pressures: Wheels<f32>,
    pub weather: Weather,
    pub track_name: String,
    pub session_type: SessionType,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weather {
    pub ambient_temp: u8,
    pub track_temp: u8,
    pub clouds: u8,
    pub rain_level: u8,
    pub wetness: u8,
}

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
