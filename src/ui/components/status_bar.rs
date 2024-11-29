use dioxus::prelude::*;

use crate::actors::ui::SessionInfo;

#[component]
pub fn StatusBar(connected: bool) -> Element {
    let info: SyncSignal<SessionInfo> = use_context();

    if info.read().live {
        rsx! {
            div { class: "grid grid-cols-2 bg-base px-4 py-2 rounded-lg content-center",
                div { class: "justify-self-start",
                    div { "{info.read().name}" }
                }
                div { class: "grid grid-cols-2 justify-self-end gap-4",
                    div { "{info.read().weather.ambient_temp} C" }
                    div { "{info.read().weather.track_temp} C" }
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
