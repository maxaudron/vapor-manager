use std::str::FromStr;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

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
pub fn ThemeSwitcher(mut theme: Signal<Theme>) -> Element {
    let active_theme = *theme.read();
    rsx! {
        select {
            class: "select select-bordered",
            oninput: move |event| {
                *theme.write() = Theme::from_str(&event.value()).unwrap();
            },
            {
                Theme::iter().map(|t| {
                    rsx! {
                        option { class: "p-2", selected: t == active_theme, "{t}" }
                    }
                })
            }
        }
    }
}
