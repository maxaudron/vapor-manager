use std::str::FromStr;

use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{setup::SetupManager, telemetry::{broadcast::{BroadcastInboundMessage, RaceSessionType, RealtimeUpdate, SessionPhase, TrackData}, Wheels}, State, StateChange};

#[component]
pub fn Debug() -> Element {
    let mut state: Signal<State> = use_context();
    let _setup_manager: Signal<SetupManager> = use_context();
    let broadcast_debug: Coroutine<BroadcastInboundMessage> = use_coroutine_handle();

    let state_change = use_coroutine_handle::<StateChange>();

    let (fl, fr, rl, rr) = {
        let state = state.read();
        (
            state.avg_tyre_pressures.front_left,
            state.avg_tyre_pressures.front_right,
            state.avg_tyre_pressures.rear_left,
            state.avg_tyre_pressures.rear_right,
        )
    };

    rsx! {
        div { class: "grid auto-rows-min bg-base rounded-md shadow-lg p-4 mx-2 gap-4 overflow-scroll",
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
                h1 { "Tyre Pressure" }
                div { class: "grid grid-cols-4 gap-4",
                    label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                        span { class: "label-text", "FL {fl:03.1}" }
                        input {
                            class: "",
                            r#type: "range",
                            min: "23.0",
                            max: "30.0",
                            step: "0.1",
                            class: "range",
                            value: "{fl}",
                            oninput: move |event| {
                                state_change
                                    .send(
                                        StateChange::AvgTyrePressure(Wheels::<f32> {
                                            front_left: event.value().parse().unwrap(),
                                            front_right: fr,
                                            rear_left: rl,
                                            rear_right: rr,
                                        }),
                                    )
                            }
                        }
                    }
                    label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                        span { class: "label-text", "FR {fr:03.1}" }
                        input {
                            class: "",
                            r#type: "range",
                            min: "23.0",
                            max: "30.0",
                            step: "0.1",
                            class: "range",
                            value: "{fr}",
                            oninput: move |event| {
                                state_change
                                    .send(
                                        StateChange::AvgTyrePressure(Wheels::<f32> {
                                            front_left: fl,
                                            front_right: event.value().parse().unwrap(),
                                            rear_left: rl,
                                            rear_right: rr,
                                        }),
                                    )
                            }
                        }
                    }
                    label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                        span { class: "label-text", "RL {rl:03.1}" }
                        input {
                            class: "",
                            r#type: "range",
                            min: "23.0",
                            max: "30.0",
                            step: "0.1",
                            class: "range",
                            value: "{rl}",
                            oninput: move |event| {
                                state_change
                                    .send(
                                        StateChange::AvgTyrePressure(Wheels::<f32> {
                                            front_left: fl,
                                            front_right: fr,
                                            rear_left: event.value().parse().unwrap(),
                                            rear_right: rr,
                                        }),
                                    )
                            }
                        }
                    }
                    label { class: "label cursor-pointer bg-surface0 rounded-md h-min px-2",
                        span { class: "label-text", "RR {rr:03.1}" }
                        input {
                            class: "",
                            r#type: "range",
                            min: "23.0",
                            max: "30.0",
                            step: "0.1",
                            class: "range",
                            value: "{rr}",
                            oninput: move |event| {
                                state_change
                                    .send(
                                        StateChange::AvgTyrePressure(Wheels::<f32> {
                                            front_left: fl,
                                            front_right: fr,
                                            rear_left: rl,
                                            rear_right: event.value().parse().unwrap(),
                                        }),
                                    )
                            }
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
                form { class: "grid grid-cols-7", onsubmit: move |event| {
                    let values = event.values();
                    let session_type = RaceSessionType::from_str(&values.get("session_type").unwrap().as_value()).unwrap();
                    let phase = SessionPhase::from_str(&values.get("phase").unwrap().as_value()).unwrap();
                    let session_end_time = values.get("session_end_time").unwrap().as_value().parse().unwrap();
                    let ambient_temp = values.get("ambient_temp").unwrap().as_value().parse().unwrap();
                    let track_temp = values.get("track_temp").unwrap().as_value().parse().unwrap();

                    broadcast_debug.send(BroadcastInboundMessage::RealtimeUpdate(RealtimeUpdate { 
                        session_type,
                        phase,
                        session_end_time,
                        ambient_temp,
                        track_temp,
                        ..Default::default()
                    }));
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
        }
    }
}
