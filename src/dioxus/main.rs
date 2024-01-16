#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    println!("Hello, Dioxus!");
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}


#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[inline_props]
fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let heading_text = "Hello, Dioxus!";
    let intro_text = "This is a brief TODO app to test out the functionality of Dioxus in building a desktop native app.";

    cx.render(rsx! {
        h1 {
            heading_text
        }
        div {
            padding: "0.5rem",
            intro_text
        }
        Link {
            to: Route::Blog {
                id: *count.get()
            },
            "Go to blog post #{count} "
        }
        div {
            h2 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }

        }
    })
}
