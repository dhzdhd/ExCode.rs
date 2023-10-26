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
        script {
            src: "https://cdnjs.cloudflare.com/ajax/libs/ace/1.31.0/ace.js",
            // integrity: "sha512-LgXIxfP1ToX6a98TRhwS7iEd6Y9mtnP89A2Ex7ehyDUWnt4OFxoX5rIVSSo+NqzUoy5Te49UV+E4kh9IIviLwQ==",
            crossorigin: "anonymous",

        }
        script {
            src: "https://cdnjs.cloudflare.com/ajax/libs/ace/1.31.0/mode-javascript.min.js",
            // integrity: "sha512-ZxGMf7jYJjId6DxujolfBm1Fgk3eNuujx2bg1oFB6jlXhj33BR47Pnh4wLRhvdCpwoWCKP23sdLQPrIBlTEFKA==",
            crossorigin: "anonymous",

        }
        script {
            defer: true,
            r#"
                var editor = ace.edit("editor");
                editor.setTheme("ace/theme/monokai");
                editor.session.setMode("ace/mode/javascript");
            "#
        }
        // Add monospaced font to fix cursor

        input {
            id: "editor",
            class: "editor"
        }
    })
}
