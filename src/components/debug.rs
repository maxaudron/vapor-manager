use dioxus::prelude::*;

use crate::{setup::SetupManager, telemetry::Wheels, State, StateChange};

#[component]
pub fn Debug() -> Element {
    let mut state: Signal<State> = use_context();
    let mut setup_manager: Signal<Option<SetupManager>> = use_context();

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
        div { class: "grid grid-rows-3 bg-base rounded-md shadow-lg p-4 mx-2 gap-4",
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
    }
}
