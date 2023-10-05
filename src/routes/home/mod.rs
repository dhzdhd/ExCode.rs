use crate::components::header::Header;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        Header {}
    })
}
