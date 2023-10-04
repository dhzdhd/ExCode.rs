#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use sir::{css, global_css, AppStyle};

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let container = css!(
        "
        background-color: red;
        color: blue;
    "
    );

    cx.render(rsx! {
        AppStyle {}
        h1 { class: {container}, "High-Fivse counter: {count}" }
        h1 { "hi" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}
