use dioxus::prelude::*;

use crate::ui::{
    components::{fuel_calculator::FuelCalculator, laps::Laps, setups::SetupView, Settings, StatusBar},
    Route,
};

#[component]
pub fn Base() -> Element {
    let route = use_route::<Route>();

    let settings = use_context::<Signal<Settings>>();
    let theme = settings.read().theme;
    let theme_lower = format!("{theme:?}").to_lowercase();

    rsx! {
        div {
            class: "h-[100vh] w-[100vw] grid grid-rows-[auto_minmax(0,_1fr)] gap-2 p-2 {theme_lower} bg-crust",
            "data-theme": "{theme:?}",
            div { class: "grid grid-cols-[max-content_1fr_max-content] gap-4",
                div { class: "justify-self-start",
                    ul { class: "menu menu-horizontal gap-2 p-0",
                        li {
                            Link {
                                class: if (route == Route::Home {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Home {},
                                "Home"
                            }
                        }
                        li {
                            Link {
                                class: if (route == Route::Setups {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Setups {},
                                "Setups"
                            }
                        }
                        // DebugLink { route: route.clone() }
                    }
                }
                StatusBar { connected: true, }
                div { class: "justify-self-end",
                    ul { class: "menu menu-horizontal gap-2 p-0",
                        li {
                            Link {
                                class: if (route == Route::SettingsComponent {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::SettingsComponent {},
                                "Settings"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}

// #[component]
// fn DebugLink(route: Route) -> Element {
//     #[cfg(debug_assertions)]
//     rsx! {
//         li {
//             Link {
//                 class: if (route == Route::Debug {}) {
//                     "btn btn-active-primary"
//                 } else {
//                     "btn bg-base border-base"
//                 },
//                 to: Route::Debug {},
//                 "Debug"
//             }
//         }
//     }

//     #[cfg(not(debug_assertions))]
//     rsx! {}
// }

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "grid grid-cols-[auto_max-content] gap-2",
            Laps {}
            div { class: "grid grid-rows-[max-content_1fr] gap-2", FuelCalculator {} }
        }
    }
}

#[component]
pub fn Setups() -> Element {
    rsx! {
        div { class: "grid grid-cols-[auto_max-content] gap-2",
            SetupView {}
            div { class: "grid grid-rows-[max-content_1fr] gap-2", FuelCalculator {} }
        }
    }
}
