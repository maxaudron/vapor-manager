use dioxus::prelude::*;

use crate::{telemetry::broadcast::LapType, State};

#[component]
pub fn Laps() -> Element {
    let state: Signal<State> = use_context();

    if state.read().laps_times.is_empty() {
        rsx! {
            div { class: "grid bg-base rounded-lg shadow-lg overflow-auto h-auto",
                div { class: "justify-self-center align-middle", "No Laps Recorded" }
            }
        }
    } else {
        let sectors = state.read().laps_times.first().unwrap().sectors.len();

        rsx! {
            div { class: "bg-base rounded-lg shadow-lg scrollable h-auto",
                table { class: "table table-zebra table-pin-rows table-sm",
                    col {}
                    col {}
                    { (0..sectors).map(|_| {
                        rsx! { col {} }
                    })},
                    colgroup { span: "3",
                        col { class: "w-1" }
                        col { class: "w-1" }
                        col { class: "w-20" }
                    }
                    colgroup { span: "3",
                        col { class: "w-1" }
                        col { class: "w-1" }
                        col { class: "w-20" }
                    }
                    colgroup { span: "3",
                        col { class: "w-1" }
                        col { class: "w-1" }
                        col { class: "w-20" }
                    }
                    thead { class: "text-md text-text",
                        tr {
                            th { scope: "col" }
                            th { scope: "col" }
                            { (0..sectors).map(|_| {
                                rsx! { th { scope: "col" } }
                            })},
                            th { scope: "colgroup", colspan: "3", "Tyre Pressure" }
                            th { scope: "colgroup", colspan: "3", "Tyre Temperature" }
                            th { scope: "colgroup", colspan: "3", "Brake Temperature" }
                        }
                        tr {
                            th { scope: "col", "#" }
                            th { scope: "col", "Laptime" }
                            { (0..sectors).map(|i| {
                                let s = format!("S{}", i+1);
                                rsx! { th { scope: "col", "{s}" } }
                            })},
                            th { scope: "col", "Min" }
                            th { scope: "col", "Avg" }
                            th { scope: "col", "Max" }
                            th { scope: "col", "Min" }
                            th { scope: "col", "Avg" }
                            th { scope: "col", "Max" }
                            th { scope: "col", "Min" }
                            th { scope: "col", "Avg" }
                            th { scope: "col", "Max" }
                        }
                    }
                    tbody {
                        {
                            state.read().laps_times.iter().zip(state.read().laps_wheels.iter()).map(|(times, lap)| {
                                rsx! {
                                    { if times.lap_type == LapType::Outlap {
                                        rsx! {
                                            tr {
                                                th { "Pit" }
                                            }
                                        }
                                    } else { rsx! {} }}
                                    tr {
                                        th { "{times.number}" }
                                        td { class: if !times.valid { "text-red" }, "{times.time}" }
                                        { times.sectors.iter().map(|sector| {
                                            rsx! {
                                                td { "{sector}" }
                                            }
                                        })}
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.min.front_left)}", "{lap.tyre_pressure.min.front_left:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.min.front_right)}", "{lap.tyre_pressure.min.front_right:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.min.rear_left)}", "{lap.tyre_pressure.min.rear_left:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.min.rear_right)}", "{lap.tyre_pressure.min.rear_right:.1}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.avg.front_left)}", "{lap.tyre_pressure.avg.front_left:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.avg.front_right)}", "{lap.tyre_pressure.avg.front_right:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.avg.rear_left)}", "{lap.tyre_pressure.avg.rear_left:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.avg.rear_right)}", "{lap.tyre_pressure.avg.rear_right:.1}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.max.front_left)}", "{lap.tyre_pressure.max.front_left:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.max.front_right)}", "{lap.tyre_pressure.max.front_right:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.max.rear_left)}", "{lap.tyre_pressure.max.rear_left:.1}" }
                                                div { style: "{tyre_pressure_color(lap.tyre_pressure.max.rear_right)}", "{lap.tyre_pressure.max.rear_right:.1}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.min.front_left)}", "{lap.tyre_temperature.min.front_left:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.min.front_right)}", "{lap.tyre_temperature.min.front_right:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.min.rear_left)}", "{lap.tyre_temperature.min.rear_left:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.min.rear_right)}", "{lap.tyre_temperature.min.rear_right:.0}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.avg.front_left)}", "{lap.tyre_temperature.avg.front_left:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.avg.front_right)}", "{lap.tyre_temperature.avg.front_right:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.avg.rear_left)}", "{lap.tyre_temperature.avg.rear_left:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.avg.rear_right)}", "{lap.tyre_temperature.avg.rear_right:.0}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.max.front_left)}", "{lap.tyre_temperature.max.front_left:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.max.front_right)}", "{lap.tyre_temperature.max.front_right:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.max.rear_left)}", "{lap.tyre_temperature.max.rear_left:.0}" }
                                                div { style: "{tyre_temperature_color(lap.tyre_temperature.max.rear_right)}", "{lap.tyre_temperature.max.rear_right:.0}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{brake_temperature_color(lap.brake_temperature.min.front_left)}", "{lap.brake_temperature.min.front_left:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.min.front_right)}", "{lap.brake_temperature.min.front_right:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.min.rear_left)}", "{lap.brake_temperature.min.rear_left:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.min.rear_right)}", "{lap.brake_temperature.min.rear_right:.0}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{brake_temperature_color(lap.brake_temperature.avg.front_left)}", "{lap.brake_temperature.avg.front_left:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.avg.front_right)}", "{lap.brake_temperature.avg.front_right:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.avg.rear_left)}", "{lap.brake_temperature.avg.rear_left:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.avg.rear_right)}", "{lap.brake_temperature.avg.rear_right:.0}" }
                                            }
                                        }
                                        td {
                                            div { class: "grid grid-cols-[min-content_min-content] grid-rows-2 gap-x-1 text-xs",
                                                div { style: "{brake_temperature_color(lap.brake_temperature.max.front_left)}", "{lap.brake_temperature.max.front_left:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.max.front_right)}", "{lap.brake_temperature.max.front_right:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.max.rear_left)}", "{lap.brake_temperature.max.rear_left:.0}" }
                                                div { style: "{brake_temperature_color(lap.brake_temperature.max.rear_right)}", "{lap.brake_temperature.max.rear_right:.0}" }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

fn factor_range(value: f32, min: f32, max: f32, ideal_min: f32, ideal_max: f32) -> f32 {
    let value = value.clamp(min, max);
    if value >= ideal_min && value <= ideal_max {
        0.5
    } else if value < ideal_min {
        ((0.0 - 0.5) / (min - ideal_min)) * (value - min)
    } else {
        ((0.5 - 1.0) / (ideal_max - max)) * (value - ideal_max) + 0.5
    }
}

fn hue(range: f32, value: f32) -> String {
    let hue = (range * (1.0 - value)).round() as i32;
    format!("color: hsl({hue}, 55%, 70%)")
}

fn tyre_pressure_color(pressure: f32) -> String {
    hue(225.0, factor_range(pressure, 22.0, 30.0, 26.0, 27.2))
}

fn tyre_temperature_color(pressure: f32) -> String {
    hue(225.0, factor_range(pressure, 50.0, 110.0, 80.0, 90.0))
}

fn brake_temperature_color(pressure: f32) -> String {
    hue(225.0, factor_range(pressure, 200.0, 800.0, 300.0, 650.0))
}
