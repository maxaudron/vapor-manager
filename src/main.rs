#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(non_snake_case)]

use std::time::Duration;

use dioxus::{
    desktop::{
        tao::{platform::windows::IconExtWindows, window::Icon},
        Config, LogicalSize, WindowBuilder,
    },
    prelude::*,
};
use futures_util::stream::StreamExt;

use telemetry::{broadcast::LapTimeData, LapWheels, SessionType};

pub mod setup;
pub mod telemetry;

// mod widgets;
pub mod components;

use components::{
    base::{Base, Home, Setups},
    debug::Debug,
    settings::Settings,
};
use tracing::debug;

use crate::{
    components::theme::Theme,
    setup::{SetupChange, SetupManager},
    telemetry::{
        broadcast::{BroadcastMsg, BroadcastState, LapType},
        Telemetry,
    },
};

pub static PROGRAM_NAME: &'static str = "Vapor Manager";

#[derive(Debug, Clone, PartialEq)]
pub enum StateChange {
    Weather(Weather),
    TrackName(String),
    SessionType(SessionType),
    ShmConnected(bool),
    BroadcastConnected(bool),
    LapWheels(LapWheels),
    LapTimeData(LapTimeData),
    Reset,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub debug: bool,
    pub shm_connected: bool,
    pub broadcast_connected: bool,
    pub weather: Weather,
    pub track_name: String,
    pub session_type: SessionType,
    pub laps_wheels: Vec<LapWheels>,
    pub laps_times: Vec<LapTimeData>,
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
        #[route("/setups")]
        Setups {},
        #[route("/settings")]
        Settings {},
        #[route("/debug")]
        #[cfg(debug_assertions)]
        Debug {},
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(windows)]
    const _TAILWIND_URL: &str = manganis::mg!(file("public\\tailwind.css"));
    #[cfg(not(windows))]
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

    let mut config = Config::new().with_disable_context_menu(true);

    #[cfg(windows)]
    {
        let mut documents =
            known_folders::get_known_folder_path(known_folders::KnownFolder::Documents).unwrap();
        documents.push(PROGRAM_NAME);
        documents.push("webview");
        config = config.with_data_directory(documents)
    }

    let bin: &[u8] = include_bytes!("../icons/icon.bin");
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
    let theme = use_context_provider(|| Signal::new(Theme::Mocha));
    let settings: Signal<Settings> = use_context_provider(|| Signal::new(Settings::init(theme)));

    #[cfg(any(not(windows), feature = "debugger"))]
    #[cfg(debug_assertions)]
    let _broadcast_debugger = use_coroutine(|rx| async move {
        telemetry::broadcast::BroadcastDebugger::coroutine(rx).await;
    });

    let setup_state: Signal<SetupManager> =
        use_context_provider(|| Signal::new(SetupManager::default()));
    let setup_manager: Coroutine<SetupChange> = use_coroutine(|rx| async move {
        to_owned![setup_state];
        SetupManager::coroutine(rx, setup_state, settings).await;
    });

    let (broadcast_tx, mut broadcast_rx) = futures_channel::mpsc::unbounded();
    let broadcast_tx_2 = broadcast_tx.clone();
    let state = use_context_provider(|| Signal::new(State::default()));
    let state_manager: Coroutine<StateChange> = use_coroutine(|mut rx| async move {
        to_owned![state];
        while let Some(msg) = rx.next().await {
            match msg {
                StateChange::Weather(weather) => state.write().weather = weather,
                StateChange::TrackName(name) => state.write().track_name = name,
                StateChange::SessionType(session) => state.write().session_type = session,
                StateChange::ShmConnected(connected) => {
                    debug!("recv shm connected: {:?}", connected);
                    state.write().shm_connected = connected;
                    if connected {
                        broadcast_tx.unbounded_send(BroadcastMsg::Connect).unwrap();
                    } else {
                        broadcast_tx
                            .unbounded_send(BroadcastMsg::Disconnect)
                            .unwrap();
                    }
                }
                StateChange::BroadcastConnected(connected) => {
                    debug!("recv broadcast connected: {:?}", connected);
                    if connected {
                        state.write().broadcast_connected = connected;
                    } else {
                        state.write().broadcast_connected = connected;
                        broadcast_tx.unbounded_send(BroadcastMsg::Aborted).unwrap();
                    }
                }
                StateChange::LapWheels(lap) => {
                    state.write().laps_wheels.push(lap);
                }
                StateChange::LapTimeData(lap) => {
                    debug!("got laptime data: {:?}", lap);
                    state.write().laps_times.push(lap);
                    let state = state.read();
                    let num_valid_laps = state
                        .laps_times
                        .iter()
                        .filter(|lap| lap.valid && lap.lap_type == LapType::Regular)
                        .count();
                    if num_valid_laps > 1 {
                        let avg_laptime = state
                            .laps_times
                            .iter()
                            .filter(|lap| lap.valid && lap.lap_type == LapType::Regular)
                            .fold(Duration::default(), |mut sum, lap| {
                                sum += lap.time.duration();
                                sum
                            });

                        let avg_laptime = avg_laptime / num_valid_laps as u32;
                        setup_manager.send(SetupChange::LapTime(avg_laptime.into()))
                    }
                }
                StateChange::Reset => {
                    let mut state = state.write();
                    state.laps_wheels = Vec::new();
                    state.laps_times = Vec::new();
                }
            }
        }
    });

    let state_manager_tx = state_manager.tx();
    let setup_manager_tx = setup_manager.tx();
    use_hook(move || {
        std::thread::spawn(move || {
            let mut telemetry = Telemetry::new(state_manager_tx, setup_manager_tx);
            telemetry.run()
        });
    });

    let state_manager_tx = state_manager.tx();
    let setup_manager_tx = setup_manager.tx();
    let _broadcast: Coroutine<BroadcastMsg> = use_coroutine(move |_| async move {
        let mut handle = None;
        let mut inner_tx = None;
        let mut connected = false;

        while let Some(msg) = broadcast_rx.next().await {
            match msg {
                BroadcastMsg::Connect => {
                    debug!("received broadcast api connect");
                    if handle.is_none() {
                        to_owned![state_manager_tx, setup_manager_tx];
                        let (tx, rx) = futures_channel::mpsc::unbounded();
                        inner_tx = Some(tx);
                        handle = Some(tokio::spawn(async move {
                            let mut broadcast =
                                BroadcastState::new(state_manager_tx, setup_manager_tx, rx);
                            broadcast.run().await;
                        }));
                        connected = true;
                    }
                }
                BroadcastMsg::Disconnect => {
                    debug!("received broadcast api disconnect");
                    connected = false;
                    if let Some(tx) = inner_tx.as_ref() {
                        tx.unbounded_send(BroadcastMsg::Disconnect).unwrap();
                        inner_tx = None;
                    }
                }
                BroadcastMsg::Aborted => {
                    debug!("received broadcast api abort");
                    if let Some(inner) = handle {
                        inner.await.unwrap();
                        handle = None;
                        inner_tx = None;
                        if connected {
                            tokio::time::sleep(Duration::from_millis(500)).await;
                            broadcast_tx_2
                                .unbounded_send(BroadcastMsg::Connect)
                                .unwrap();
                        }
                    }
                }
            }
        }
    });

    rsx! {
        Router::<Route> {}
    }
}
