use std::time::Duration;

use dioxus::prelude::*;

use crate::setup::{SetupChange, SetupManager};

#[component]
pub fn FuelCalculator() -> Element {
    let setup_manager: Signal<SetupManager> = use_context();
    let setup_manager_tx = use_coroutine_handle::<SetupChange>();

    rsx! {
        div { class: "grid auto-rows-min bg-base rounded-lg shadow-lg",
            div { class: "label p-0 border-b-[1px] border-crust",
                span { class: "label-text text-nowrap p-4", "Fuel/Lap" }
                { if setup_manager.read().fuel_per_lap > 0.0 {
                    rsx! { span { class: "label-text text-nowrap p-4", "{setup_manager.read().fuel_per_lap:.2} l" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap p-4 text-red", "Drive Lap" } }
                }
                }
            }
            div { class: "label p-0 border-b-[1px] border-crust",
                span { class: "label-text text-nowrap p-4", "Best Lap" }
                { if setup_manager.read().best_lap.millis > 0 {
                    rsx! { span { class: "label-text text-nowrap p-4", "{setup_manager.read().best_lap.text}" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap p-4 text-red", "Drive Lap" } }
                }
                }
            }
            div { class: "grid auto-rows-min border-b-[1px] border-crust",
                div { class: "label p-0 h-min",
                    span { class: "label-text text-nowrap p-4", "Race Duration" }
                    input {
                        r#type: "number",
                        // 4rem
                        class: "input input-bordered input-nospinner w-32 pr-[4rem] h-9",
                        min: "0",
                        max: "{u64::MAX}",
                        step: "1",
                        value: "{setup_manager.read().session_length.as_secs() / 60}",
                        oninput: move |event| {
                            if event.value() != "" {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength(
                                            Duration::from_secs(event.value().parse::<u64>().unwrap() * 60),
                                        ),
                                    )
                            }
                        }
                    }
                    span { class: "ml-[-3rem] mr-[1.3rem]", "mins" }
                }
                ul { class: "menu menu-horizontal rounded-box gap-2 w-max pt-0",
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx.send(SetupChange::SessionLength(Duration::from_secs(25 * 60)))
                            },
                            "25 mins"
                        }
                    }
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx.send(SetupChange::SessionLength(Duration::from_secs(45 * 60)))
                            },
                            "45 mins"
                        }
                    }
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx.send(SetupChange::SessionLength(Duration::from_secs(65 * 60)))
                            },
                            "65 mins"
                        }
                    }
                }
            }
            div { class: "label p-0",
                span { class: "label-text text-nowrap p-4", "Fuel Required" }
                { if setup_manager.read().fuel > 0 {
                    rsx! { span { class: "label-text text-nowrap text-green p-4", "{setup_manager.read().fuel} l" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap p-4 text-red", "Enter Duration" } }
                }
                }
            }
            div { class: "label p-0",
                span { class: "label-text text-nowrap p-4 pt-0", "Reserve Fuel" }
                span { class: "label-text text-nowrap text-green p-4 pt-0",
                    "{setup_manager.read().reserve_laps} Laps"
                }
                { if setup_manager.read().reserve_fuel_l > 0.0 {
                    rsx! { span { class: "label-text text-nowrap text-green p-4 pt-0", "{setup_manager.read().reserve_fuel_l:.1} l" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap p-4 pt-0 text-red", "0.0 l" } }
                }
                }
            }
        }
    }
}
