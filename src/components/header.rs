use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header {
            class: "header--home",
            "Hello, world!"
        }
    })
}
