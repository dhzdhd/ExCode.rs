#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}
