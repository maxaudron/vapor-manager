use std::time::Duration;

use dioxus::prelude::*;

use crate::{
    setup::{SetupChange, SetupManager},
    telemetry::broadcast::RaceSessionType,
};

#[component]
pub fn FuelCalculator() -> Element {
    let setup_manager: Signal<SetupManager> = use_context();
    let setup_manager_tx = use_coroutine_handle::<SetupChange>();

    rsx! {
        div { class: "grid auto-rows-min bg-base rounded-lg shadow-lg",
            div { class: "label px-0 py-2 border-b-[1px] border-crust",
                span { class: "label-text text-nowrap px-4", "Fuel/Lap" }
                { if setup_manager.read().fuel_per_lap > 0.0 {
                    rsx! { span { class: "label-text text-nowrap px-4", "{setup_manager.read().fuel_per_lap:.2} l" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap px-4 text-red", "Drive Lap" } }
                }
                }
            }
            div { class: "label px-0 py-2 border-b-[1px] border-crust",
                span { class: "label-text text-nowrap px-4", "Avg Lap" }
                { if setup_manager.read().avg_lap.duration().as_millis() > 0 {
                    rsx! { span { class: "label-text text-nowrap px-4", "{setup_manager.read().avg_lap}" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap px-4 text-red", "Drive Lap" } }
                }
                }
            }
            div { class: "grid auto-rows-min border-b-[1px] border-crust",
                div { class: "label px-0 py-2 h-min",
                    span { class: "label-text text-nowrap px-4", "Quali Duration" }
                    input {
                        r#type: "number",
                        // 4rem
                        class: "input input-bordered input-nospinner w-[6.5rem] pr-[3rem] pl-3 h-9",
                        min: "0",
                        max: "{u64::MAX}",
                        step: "1",
                        value: "{setup_manager.read().qualifying_length.as_secs() / 60}",
                        oninput: move |event| {
                            if event.value() != "" {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Qualifying,
                                            Duration::from_secs(event.value().parse::<u64>().unwrap() * 60),
                                        )),
                                    )
                            }
                        }
                    }
                    span { class: "ml-[-4.8rem] mr-[0.8rem]", "mins" }
                }
                ul { class: "menu menu-horizontal rounded-box gap-2 w-max pt-0",
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Qualifying,
                                            Duration::from_secs(5 * 60),
                                        )),
                                    )
                            },
                            "5 mins"
                        }
                    }
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Qualifying,
                                            Duration::from_secs(10 * 60),
                                        )),
                                    )
                            },
                            "10 mins"
                        }
                    }
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Qualifying,
                                            Duration::from_secs(15 * 60),
                                        )),
                                    )
                            },
                            "15 mins"
                        }
                    }
                }
            }
            div { class: "grid auto-rows-min border-b-[1px] border-crust",
                div { class: "label p-0 h-min",
                    span { class: "label-text text-nowrap p-4", "Race Duration" }
                    input {
                        r#type: "number",
                        // 4rem
                        class: "input input-bordered input-nospinner w-[6.5rem] pr-[3rem] pl-3 h-9",
                        min: "0",
                        max: "{u64::MAX}",
                        step: "1",
                        value: "{setup_manager.read().race_length.as_secs() / 60}",
                        oninput: move |event| {
                            if event.value() != "" {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Race,
                                            Duration::from_secs(event.value().parse::<u64>().unwrap() * 60),
                                        )),
                                    )
                            }
                        }
                    }
                    span { class: "ml-[-4.8rem] mr-[0.8rem]", "mins" }
                }
                ul { class: "menu menu-horizontal rounded-box gap-2 w-max pt-0",
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Race,
                                            Duration::from_secs(25 * 60),
                                        )),
                                    )
                            },
                            "25 mins"
                        }
                    }
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Race,
                                            Duration::from_secs(45 * 60),
                                        )),
                                    )
                            },
                            "45 mins"
                        }
                    }
                    li {
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                setup_manager_tx
                                    .send(
                                        SetupChange::SessionLength((
                                            RaceSessionType::Race,
                                            Duration::from_secs(65 * 60),
                                        )),
                                    )
                            },
                            "65 mins"
                        }
                    }
                }
            }
            div { class: "label p-0 pt-2 pb-1",
                span { class: "label-text text-nowrap px-4", "Race Fuel" }
                { if setup_manager.read().race_fuel > 0 {
                    rsx! { span { class: "label-text text-nowrap text-green px-4", "{setup_manager.read().race_fuel} l" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap px-4 text-red", "Enter Duration" } }
                }
                }
            }
            div { class: "label p-0 pb-2 pt-1",
                span { class: "label-text text-nowrap px-4", "Quali Fuel" }
                { if setup_manager.read().race_fuel > 0 {
                    rsx! { span { class: "label-text text-nowrap text-green px-4", "{setup_manager.read().qualifying_fuel} l" } }
                } else {
                    rsx! { span { class: "label-text text-nowrap px-4 text-red", "Enter Duration" } }
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
