use dioxus::prelude::*;

// pub fn onclick<E, T>(
//     _f: impl FnMut(Event<MouseData>) -> E + 'static
// ) -> Attribute
// where
//     E: EventReturn<T>,

#[component]
pub fn InputNumber<T: 'static + std::cmp::PartialEq>(
    name: String,
    value: Signal<T>,
    min: T,
    max: T,
    step: T,
) -> Element
where
    T: Copy
        + 'static
        + std::ops::Add
        + std::ops::Sub
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::fmt::Display
        + std::str::FromStr
        + std::cmp::PartialEq
        + std::cmp::PartialOrd,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    Signal<T>: std::ops::AddAssign<T> + std::ops::SubAssign<T>,
{
    rsx! {
        div { class: "label bg-surface0 rounded-md h-min px-2",
            span { class: "text-lg pl-8 label-text text-nowrap", "{name}" }
            div { class: "label p-0 w-min",
                button {
                    class: "btn btn-ghost m-0 p-0",
                    onclick: move |_| {
                        if value() > min {
                            value -= step;
                        }
                    },
                    svg {
                        class: "svg",
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 16 16",
                        path {
                            "fill-rule": "evenodd",
                            d: "M9.224 1.553a.5.5 0 0 1 .223.67L6.56 8l2.888 5.776a.5.5 0 1 1-.894.448l-3-6a.5.5 0 0 1 0-.448l3-6a.5.5 0 0 1 .67-.223"
                        }
                    }
                }
                input {
                    r#type: "number",
                    class: "input-number",
                    min: "{min}",
                    max: "{max}",
                    step: "1",
                    value: "{value}",
                    oninput: move |event| {
                        if event.value() != "" {
                            value.set(event.value().parse::<T>().unwrap());
                        }
                    }
                }
                button {
                    class: "btn btn-ghost m-0 p-0",
                    onclick: move |_| {
                        if value() < max {
                            value += step;
                        }
                    },
                    svg {
                        class: "svg",
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 16 16",
                        path {
                            "fill-rule": "evenodd",
                            d: "M6.776 1.553a.5.5 0 0 1 .671.223l3 6a.5.5 0 0 1 0 .448l-3 6a.5.5 0 1 1-.894-.448L9.44 8 6.553 2.224a.5.5 0 0 1 .223-.671"
                        }
                    }
                }
            }
        }
    }
}
