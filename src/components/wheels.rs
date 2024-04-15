use dioxus::prelude::*;

use crate::State;

#[component]
pub fn WheelPressures() -> Element {
    let state: Signal<State> = use_context();
    let telemetry = state.read();
    let wheels = telemetry.avg_tyre_pressures.clone();

    rsx! {
        div { class: "bg-base rounded-md shadow-lg p-4 grid auto-rows-min w-min h-min justify-items-center",
            h1 { class: "text-xl text-nowrap pb-4", "Tyre Pressures" }
            div { class: "grid grid-cols-[repeat(4,_min-content)] gap-4",
                Wheel { pressure: wheels.front_left, name: "FL" }
                Wheel { pressure: wheels.front_right, name: "FR" }
                Wheel { pressure: wheels.rear_left, name: "RL" }
                Wheel { pressure: wheels.rear_right, name: "RR" }
            }
        }
    }
}

#[component]
pub fn Wheel(pressure: f32, name: String) -> Element {
    let percent = pressure_optimum_factor(pressure);
    let color = pressure_color(percent);
    let percent = (percent * 100.0).round() as i32;
    rsx! {
        div { class: "grid auto-rows-auto justify-items-center",
            div { class: "pb-2", "{pressure:03.1}" }
            div {
                class: "h-40 w-4 rounded-full bg-mantle shadow-sm",
                style: "position: relative;",
                div {
                    class: "rounded-full",
                    style: "position: absolute; bottom: 0; width: 100%; height: {percent}%; background-color: hsl({color}, 55%, 70%)"
                }
            }
            div { class: "pt-2", "{name}" }
        }
    }
}

// TODO this is for gt3 need to use differnet values for wet and gt4
fn pressure_optimum_factor(pressure: f32) -> f32 {
    (pressure - 22.0) / (30.0 - 22.0)
}

fn pressure_color(percentage: f32) -> i32 {
    (250.0 * (1.0 - percentage)).round() as i32
}
