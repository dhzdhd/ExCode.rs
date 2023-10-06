use crate::components::{header::Header, home::editor::Editor};
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        Header {}
        Editor {}
    })
}
