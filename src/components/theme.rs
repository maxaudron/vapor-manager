use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};
use tracing::debug;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, EnumIter)]
pub enum Theme {
    Light,
    Dark,
}

#[component]
pub fn ThemeSwitcher(mut theme: Signal<Theme>) -> Element {
    rsx! {
        div { class: "dropdown dropdown-end",
            div { tabindex: "0", role: "button", class: "btn m-1",
                "Theme"
                svg { width: "12px", height: "12px", class: "h-2 w-2 fill-current opacity-60 inline-block",
                    xmlns: "http://www.w3.org/2000/svg", "viewBox": "0 0 2048 2048",
                    path { d: "M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" }
                }
            }
            ul { tabindex: "0", class: "dropdown-content z-[1] p-2 shadow-2xl bg-base-300 rounded-box w-44",
                { Theme::iter().map(|t| {
                    let active = if theme() == t { "bg-primary text-warning-content" } else { "" };
                    rsx! {
                        li {
                            button {
                                class: "btn btn-sm btn-block btn-ghost justify-start {active}",
                                onclick: move |_| {
                                    debug!("set theme to {:?}", t);
                                    *theme.write() = t
                                },
                                "{t:?}"
                            }
                        }
                    }
                })}
            }
        }
    }
}
