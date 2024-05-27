use std::{str::FromStr, time::Duration};

use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    setup::SetupManager,
    telemetry::{
        broadcast::{
            BroadcastInboundMessage, LapTimeData, LapType, RaceSessionType, RealtimeUpdate, SessionPhase, TrackData
        },
        AvgMinMax, LapWheels, Wheels,
    },
    State, StateChange,
};

#[component]
pub fn Debug() -> Element {
    let mut state: Signal<State> = use_context();
    let _setup_manager: Signal<SetupManager> = use_context();
    let broadcast_debug: Coroutine<BroadcastInboundMessage> = use_coroutine_handle();

    let state_change = use_coroutine_handle::<StateChange>();

    rsx! {
        div { class: "grid auto-rows-min bg-base rounded-md shadow-lg p-4 gap-4 overflow-scroll",
            div { class: "grid grid-cols-3 gap-4",
                label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                    span { class: "label-text", "debug" }
                    input {
                        r#type: "checkbox",
                        class: "checkbox",
                        oninput: move |event| {
                            let is_enabled = event.value() == "true";
                            state.write().debug = is_enabled;
                        }
                    }
                }
                label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                    span { class: "label-text", "shm_connected" }
                    input {
                        r#type: "checkbox",
                        class: "checkbox",
                        oninput: move |event| {
                            let is_enabled = event.value() == "true";
                            state.write().shm_connected = is_enabled;
                        }
                    }
                }
                label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                    span { class: "label-text", "broadcast_connected" }
                    input {
                        r#type: "checkbox",
                        class: "checkbox",
                        oninput: move |event| {
                            let is_enabled = event.value() == "true";
                            state.write().broadcast_connected = is_enabled;
                        }
                    }
                }
            }
            div { class: "grid auto-rows-min",
                h1 { class: "", "Broadcast API" }
                form {
                    class: "grid grid-cols-4",
                    onsubmit: move |event| {
                        let values = event.values();
                        let name = values.get("name").unwrap().as_value();
                        let id = values.get("id").unwrap().as_value().parse().unwrap();
                        broadcast_debug
                            .send(
                                BroadcastInboundMessage::TrackData(TrackData {
                                    name: name,
                                    id: id,
                                    ..Default::default()
                                }),
                            )
                    },
                    h1 { "TrackData" }
                    input {
                        r#type: "text",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "name",
                        placeholder: "name"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "id",
                        placeholder: "id"
                    }
                    input { class: "btn btn-sm", r#type: "submit" }
                }
                form {
                    class: "grid grid-cols-7",
                    onsubmit: move |event| {
                        let values = event.values();
                        let session_type = RaceSessionType::from_str(
                                &values.get("session_type").unwrap().as_value(),
                            )
                            .unwrap();
                        let phase = SessionPhase::from_str(&values.get("phase").unwrap().as_value())
                            .unwrap();
                        let session_end_time = values
                            .get("session_end_time")
                            .unwrap()
                            .as_value()
                            .parse()
                            .unwrap();
                        let ambient_temp = values
                            .get("ambient_temp")
                            .unwrap()
                            .as_value()
                            .parse()
                            .unwrap();
                        let track_temp = values.get("track_temp").unwrap().as_value().parse().unwrap();
                        broadcast_debug
                            .send(
                                BroadcastInboundMessage::RealtimeUpdate(RealtimeUpdate {
                                    session_type,
                                    phase,
                                    session_end_time,
                                    ambient_temp,
                                    track_temp,
                                    ..Default::default()
                                }),
                            );
                    },
                    h1 { "RealtimeUpdate" }
                    select {
                        name: "session_type",
                        class: "select select-bordered select-sm bg-surface0",
                        option { disabled: true, selected: true, "session type" }
                        { RaceSessionType::iter().map(|t| {
                            rsx! { option { "{t}" } }
                        }) }
                    }
                    select {
                        name: "phase",
                        class: "select select-bordered select-sm bg-surface0",
                        option { disabled: true, selected: true, "phase" }
                        { SessionPhase::iter().map(|t| {
                            rsx! { option { "{t}" } }
                        }) }
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "session_end_time",
                        placeholder: "session end time"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "ambient_temp",
                        placeholder: "ambient temp"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "track_temp",
                        placeholder: "track temp"
                    }
                    input { class: "btn btn-sm", r#type: "submit" }
                }
            }
            div { class: "grid auto-rows-min",
                h1 { class: "", "Laps" }
                form {
                    class: "grid grid-cols-4",
                    onsubmit: move |event| {
                        let values = event.values();
                        let number = values.get("lapcount").unwrap().as_value().parse().unwrap();
                        let time = Duration::from_millis(
                                values.get("laptime").unwrap().as_value().parse().unwrap(),
                            )
                            .into();
                        let valid = values
                            .get("valid")
                            .unwrap_or(&FormValue(vec!["false".to_string()]))
                            .as_value() == "true";
                        let sector1 = Duration::from_millis(
                                values.get("sector1").unwrap().as_value().parse().unwrap(),
                            )
                            .into();
                        let sector2 = Duration::from_millis(
                                values.get("sector2").unwrap().as_value().parse().unwrap(),
                            )
                            .into();
                        let sector3 = Duration::from_millis(
                                values.get("sector3").unwrap().as_value().parse().unwrap(),
                            )
                            .into();
                        let sectors = vec![sector1, sector2, sector3];
                        let tyre_pressure: f32 = values
                            .get("lap_tyre_pressure")
                            .unwrap()
                            .as_value()
                            .parse()
                            .unwrap();
                        let tyre_pressure = Wheels {
                            front_left: tyre_pressure,
                            front_right: tyre_pressure,
                            rear_left: tyre_pressure,
                            rear_right: tyre_pressure,
                        };
                        let tyre_pressure = AvgMinMax {
                            avg: tyre_pressure,
                            min: tyre_pressure,
                            max: tyre_pressure,
                        };
                        let tyre_temperature: f32 = values
                            .get("lap_tyre_temperature")
                            .unwrap()
                            .as_value()
                            .parse()
                            .unwrap();
                        let tyre_temperature = Wheels {
                            front_left: tyre_temperature,
                            front_right: tyre_temperature,
                            rear_left: tyre_temperature,
                            rear_right: tyre_temperature,
                        };
                        let tyre_temperature = AvgMinMax {
                            avg: tyre_temperature,
                            min: tyre_temperature,
                            max: tyre_temperature,
                        };
                        let brake_temperature: f32 = values
                            .get("lap_brake_temperature")
                            .unwrap()
                            .as_value()
                            .parse()
                            .unwrap();
                        let brake_temperature = Wheels {
                            front_left: brake_temperature,
                            front_right: brake_temperature,
                            rear_left: brake_temperature,
                            rear_right: brake_temperature,
                        };
                        let brake_temperature = AvgMinMax {
                            avg: brake_temperature,
                            min: brake_temperature,
                            max: brake_temperature,
                        };
                        state_change
                            .send(
                                StateChange::LapTimeData(LapTimeData {
                                    number,
                                    sectors,
                                    time,
                                    valid,
                                    lap_type: LapType::Regular,
                                }),
                            );
                        state_change
                            .send(
                                StateChange::LapWheels(LapWheels {
                                    number,
                                    tyre_pressure,
                                    tyre_temperature,
                                    brake_temperature,
                                }),
                            );
                    },
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "lapcount",
                        value: "1",
                        placeholder: "lapcount"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "laptime",
                        value: "78400",
                        placeholder: "laptime"
                    }
                    input {
                        r#type: "checkbox",
                        class: "checkbox",
                        name: "valid",
                        value: "true",
                        placeholder: "valid"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "sector1",
                        value: "9820",
                        placeholder: "sector1"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "sector2",
                        value: "2390",
                        placeholder: "sector2"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "sector3",
                        value: "4902",
                        placeholder: "sector3"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "lap_tyre_pressure",
                        value: "27",
                        placeholder: "tyre pressure"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "lap_tyre_temperature",
                        value: "80",
                        placeholder: "tyre temperature"
                    }
                    input {
                        r#type: "number",
                        class: "input input-bordered input-sm bg-surface0",
                        name: "lap_brake_temperature",
                        value: "500",
                        placeholder: "brake temperature"
                    }
                    input { class: "btn btn-sm", r#type: "submit" }
                }
            }
        }
    }
}
