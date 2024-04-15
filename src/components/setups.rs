use dioxus::prelude::*;

use crate::{
    setup::{Setup, SetupManager},
    State, Weather,
};

#[component]
pub fn SetupManager() -> Element {
    let setup_manager: Signal<Option<SetupManager>> = use_context();
    let setup_manager = setup_manager.read();

    rsx! {
        div { class: "grid grid-rows-[min-content_auto] bg-base p-2 rounded-lg shadow-lg",
            div { class: "grid grid-cols-3",
                h1 { class: "text-md pb-2 pl-2 justify-self-start self-end", "Templates" }
                h1 { class: "text-xl pb-2 justify-self-center", "Setups" }
                h1 { class: "text-md pb-2 pr-2 justify-self-end self-end", "Adjusted" }
            }
            { if let Some(manager) = setup_manager.as_ref() {
                rsx! { div { class: "grid grid-cols-[1fr_min-content_1fr]",
                    div { class: "grid auto-rows-min",
                        { manager.setups.iter().map(|setup| { rsx! {
                            SetupSmall { 
                                name: "{setup.name}",
                                air_temp: setup.air_temperature, 
                                track_temp: setup.road_temperature 
                            }
                        }})}
                    }
                    div { class: "divider divider-horizontal h-full mx-2" }
                    div { class: "grid auto-rows-min",
                        if manager.adj_setups.is_empty() {
                            span { class: "loading loading-ring loading-lg justify-self-center self-center" }
                        } else {{
                            manager.adj_setups.iter().map(|setup| { rsx! {
                                SetupSmall { 
                                    name: "{setup.name}",
                                    air_temp: setup.air_temperature, 
                                    track_temp: setup.road_temperature 
                                }
                            }})
                        }}
                    }
                }}
            } else {
                rsx! {
                    div { class: "divider divider-horizontal h-full mx-2 justify-self-center",
                        "No Setups Loaded"
                    }
                }
            }}
        }
    }
}

#[component]
pub fn SetupSmall(name: String, air_temp: u8, track_temp: u8) -> Element {
    rsx! {
        div { class: "bg-base-300 rounded-md p-2",
            "{name}"
            div { class: "badge bg-blue-100 text-black ml-4", "{air_temp}" }
            div { class: "badge bg-gray-900 ml-2", "{track_temp}" }
        }
    }
}