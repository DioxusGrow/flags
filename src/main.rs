#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use once_cell::sync::Lazy;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Home {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        h1 { class: "p-4", "Flags:" }
        for (lang , Flag) in LANG_NAMES.iter().zip(FLAGS.iter()) {
            div { class: "",
                div { class: "px-4 py-1 flex flex-row space-x-2",
                    div { Flag {} }
                    div { "{lang}" }
                }
            }
        }
    }
}

pub const LANG_NAMES: [&str; 2] = ["English", "German"];
pub static FLAGS: Lazy<[fn() -> Element; 2]> = Lazy::new(|| [De, Sm]);

#[component]
pub fn De() -> Element {
    rsx! {
        svg { class: "h-6", id: "flag-icons-de", view_box: "0 0 640 480",
            path { fill: "#fc0", d: "M0 320h640v160H0z" }
            path { fill: "#000001", d: "M0 0h640v160H0z" }
            path { fill: "red", d: "M0 160h640v160H0z" }
        }
    }
}

#[component]
pub fn Sm() -> Element {
    rsx! {
        svg { class: "h-6", id: "flag-icons-de", view_box: "0 0 640 480",
            path { fill: "yellow", d: "M0 320h640v160H0z" }
            path { fill: "#000001", d: "M0 0h640v160H0z" }
            path { fill: "blue", d: "M0 160h640v160H0z" }
        }
    }
}
