use dioxus::prelude::*;

pub fn Settings(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "Settings"
        }
    })
}
