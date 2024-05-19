use dioxus::prelude::*;

use crate::setup::SetupManager;

#[component]
pub fn SetupView() -> Element {
    let setup_manager: Signal<SetupManager> = use_context();
    let setup_manager = setup_manager.read();

    rsx! {
        div { class: "grid grid-rows-[min-content_auto] bg-base p-2 rounded-lg shadow-lg",
            div { class: "grid grid-cols-1",
                h1 { class: "text-xl pb-2 justify-self-center", "Setups" }
            }
            { if !setup_manager.setups.is_empty() {
                rsx! { div { class: "grid grid-rows-[min-content_1fr_min-content_1fr] overflow-y-auto",
                    h1 { class: "text-md pb-2", "Templates" }
                    div { class: "grid auto-rows-min",
                        { setup_manager.setups.iter().map(|setup| { rsx! {
                            SetupSmall {
                                name: "{setup.name}",
                                air_temp: setup.air_temperature,
                                track_temp: setup.road_temperature
                            }
                        }})}
                    }
                    h1 { class: "text-md py-2", "Adjusted" }
                    div { class: "grid auto-rows-min",
                        if setup_manager.adj_setups.is_empty() {
                            span { class: "loading loading-ring loading-lg justify-self-center self-center" }
                        } else {{
                            setup_manager.adj_setups.iter().map(|setup| { rsx! {
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
                    div { class: "divider divider-vertical h-full mx-2 justify-self-center",
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
        div { class: "grid grid-rows-[min-content_min-content] bg-surface0 rounded-md p-2",
            "{name}"
            div {
                div { class: "badge bg-sky text-black", 
                    svg { class: "w-3 mr-2", xmlns: "http://www.w3.org/2000/svg", "viewBox": "0 0 512 512", 
                        path { d: "M288 32c0 17.7 14.3 32 32 32h32c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H352c53 0 96-43 96-96s-43-96-96-96H320c-17.7 0-32 14.3-32 32zm64 352c0 17.7 14.3 32 32 32h32c53 0 96-43 96-96s-43-96-96-96H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H384c-17.7 0-32 14.3-32 32zM128 512h32c53 0 96-43 96-96s-43-96-96-96H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H160c17.7 0 32 14.3 32 32s-14.3 32-32 32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32z" }
                    }
                    "{air_temp} C"
                }
                div { class: "badge bg-zinc-900 text-white ml-2", 
                    svg { class: "w-3 mr-2", xmlns: "http://www.w3.org/2000/svg", "viewBox": "0 0 576 512", 
                        path { fill: "#ffffff", d: "M256 32H181.2c-27.1 0-51.3 17.1-60.3 42.6L3.1 407.2C1.1 413 0 419.2 0 425.4C0 455.5 24.5 480 54.6 480H256V416c0-17.7 14.3-32 32-32s32 14.3 32 32v64H521.4c30.2 0 54.6-24.5 54.6-54.6c0-6.2-1.1-12.4-3.1-18.2L455.1 74.6C446 49.1 421.9 32 394.8 32H320V96c0 17.7-14.3 32-32 32s-32-14.3-32-32V32zm64 192v64c0 17.7-14.3 32-32 32s-32-14.3-32-32V224c0-17.7 14.3-32 32-32s32 14.3 32 32z" }
                    }
                    "{track_temp} C" 
                }
            }
        }
    }
}
