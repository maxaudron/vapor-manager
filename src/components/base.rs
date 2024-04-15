use dioxus::prelude::*;

use crate::{components::{setups::SetupManager, status_bar::StatusBar, theme::{Theme, ThemeSwitcher}, wheels::WheelPressures}, Route};

#[component]
pub fn Base() -> Element {
    let theme = use_signal(|| Theme::Mocha);
    let route = use_route::<Route>();

    let theme_lower = format!("{theme:?}").to_lowercase();

    rsx! {
        div {
            class: "h-[100vh] w-[100vw] grid grid-rows-[auto_minmax(0,_1fr)_auto] {theme_lower} bg-crust",
            "data-theme": "{theme:?}",
            div { class: "grid grid-cols-2",
                div { class: "justify-self-start",
                    ul { class: "menu menu-horizontal rounded-box gap-2 bg-sla",
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
                                class: if (route == Route::Settings {}) {
                                    "btn btn-active-primary"
                                } else {
                                    "btn bg-base border-base"
                                },
                                to: Route::Settings {},
                                "Settings"
                            }
                        }
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
                }
                div { class: "justify-self-end",
                    ThemeSwitcher { theme }
                }
            }
            Outlet::<Route> {}
            StatusBar {}
        }
    }
}

#[component]
pub fn Settings() -> Element {
    rsx! { "Blog post" }
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