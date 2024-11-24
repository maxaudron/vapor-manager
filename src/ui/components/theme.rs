use std::str::FromStr;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::ui::components::Settings;

#[derive(
    Clone,
    Default,
    Copy,
    Debug,
    Display,
    PartialEq,
    PartialOrd,
    EnumIter,
    EnumString,
    Deserialize,
    Serialize,
)]
pub enum Theme {
    Latte,
    Frappe,
    Macchiato,
    #[default]
    Mocha,
}

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut settings: Signal<Settings> = use_context();
    rsx! {
        select {
            class: "select select-bordered",
            oninput: move |event| {
                settings.write().theme = Theme::from_str(&event.value()).unwrap();
            },
            {
                Theme::iter().map(|t| {
                    rsx! {
                        option { class: "p-2", selected: t == settings.read().theme, "{t}" }
                    }
                })
            }
        }
    }
}
