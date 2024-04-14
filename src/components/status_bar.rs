use dioxus::prelude::*;

use crate::State;

#[component]
pub fn StatusBar() -> Element {
    let state: Signal<State> = use_context();
    let state = state.read();

    rsx! {
        div { class: "grid grid-cols-5 bg-base-200 px-4 py-2",
            div { "{state.track_name}" }
            div { "{state.weather.ambient_temp}" }
            div { "{state.weather.track_temp}" }
        }
    }
}