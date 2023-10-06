use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

pub fn Editor(cx: Scope) -> Element {
    // let editor = unsafe {
    //     edit("editor");
    // };
    // let create_eval = use_eval(cx);
    // create_eval(
    //     r#"
    //     let e = document.getElementById("editor");
    //     e.
    // "#,
    // );

    cx.render(rsx! {
        div {
            id: "editor",
            class: "editor"
        }
    })
}
