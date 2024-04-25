use dioxus::prelude::*;

use crate::State;

#[component]
pub fn StatusBar() -> Element {
    let state: Signal<State> = use_context();
    let state = state.read();

    if state.shm_connected && state.broadcast_connected {
        rsx! {
            div { class: "grid grid-cols-2 bg-base px-4 py-2 rounded-lg content-center",
                div { class: "justify-self-start", div { "{state.track_name}" } }
                div { class: "grid grid-cols-2 justify-self-end gap-4",
                    div { "{state.weather.ambient_temp} C" }
                    div { "{state.weather.track_temp} C" }
                }
            }
        }
    } else {
        rsx! {
            div { class: "grid grid-cols-1 justify-items-center content-center px-4 py-2 bg-error text-error-content rounded-lg",
                b { "No Assetto Corsa Competizione session running" }
            }
        }
    }
}
