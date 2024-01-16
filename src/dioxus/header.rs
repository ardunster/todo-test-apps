use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    let heading_text = "Hello, Dioxus!";

    cx.render(rsx! {
        h1 {
            heading_text
        }
    })
}
