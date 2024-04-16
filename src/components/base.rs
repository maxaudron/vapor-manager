use dioxus::prelude::*;

use crate::{
    components::{
        settings::Settings, setups::SetupManager, status_bar::StatusBar, theme::Theme,
        wheels::WheelPressures,
    },
    Route,
};

#[component]
pub fn Base() -> Element {
    let route = use_route::<Route>();

    let theme = use_context_provider(|| Signal::new(Theme::Mocha));
    let theme_lower = format!("{theme:?}").to_lowercase();

    let _settings: Signal<Settings> =
        use_context_provider(|| Signal::new(Settings::init(theme)));

    rsx! {
        div {
            class: "h-[100vh] w-[100vw] grid grid-rows-[auto_minmax(0,_1fr)_auto] {theme_lower} bg-crust",
            "data-theme": "{theme:?}",
            div { class: "grid grid-cols-2",
                div { class: "justify-self-start",
                    ul { class: "menu menu-horizontal rounded-box gap-2",
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
                        DebugLink { route: route.clone() }
                    }
                }
                div { class: "justify-self-end",
                    ul { class: "menu menu-horizontal rounded-box gap-2",
                        li {
                            Link {
                                class: if (route == Route::Settings {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Settings {},
                                "Settings"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
            StatusBar {}
        }
    }
}

#[component]
fn DebugLink(route: Route) -> Element {
    #[cfg(debug_assertions)]
    rsx! {
        li {
            Link {
                class: if (route == Route::Debug {}) {
                    "btn btn-active-primary"
                } else {
                    "btn bg-base border-base"
                },
                to: Route::Debug {},
                "Debug"
            }
        }
    }

    #[cfg(not(debug_assertions))]
    rsx! {  }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "grid grid-rows-[min-content_auto] gap-2 px-2",
            div { class: "grid grid-cols-[min-content_auto] gap-2 h-min",
                WheelPressures {}
                SetupManager {}
            }
            div { class: "bg-base rounded-lg shadow-lg h-auto", "aaaa" }
        }
    }
}
