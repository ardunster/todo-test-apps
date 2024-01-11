#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    println!("Hello, Dioxus!");
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, Dioxus!"
        }
    })
}
