#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIconStylesheet, MaterialIconVariant};
use dioxus_router::prelude::*;
use sir::{css, global_css, AppStyle};

mod components;
mod routes;

use routes::{home::Home, settings::Settings};

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    #[redirect("/:..segments", |segments: Vec<String>| Route::Home {})]
    Home {},
    #[route("/settings")]
    Settings {}
    // #[route("/:..segments")]
    // NotFound { segments: Vec<String> },
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    const STYLE: &str = include_str!("./styles.css");
    cx.render(rsx! {
        script {
            src: "https://cdnjs.cloudflare.com/ajax/libs/ace/1.28.0/ace.js",
            integrity: "sha512-ZxGMf7jYJjId6DxujolfBm1Fgk3eNuujx2bg1oFB6jlXhj33BR47Pnh4wLRhvdCpwoWCKP23sdLQPrIBlTEFKA==",
            defer: "true",
        }
        script {
            defer: "true",
            r#"
                var editor = ace.edit("editor");
                editor.setTheme("ace/theme/monokai");
                editor.session.setMode("ace/mode/javascript");
            "#
        }

        Router::<Route> { }
        MaterialIconStylesheet {
            variant: MaterialIconVariant::Regular
        }
        style { "{STYLE}" }
    })
}
