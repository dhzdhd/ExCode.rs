#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIconStylesheet, MaterialIconVariant};
use sir::{css, global_css, AppStyle};

// enum Route {
//     #[route("/home")]
//     #[redirect("/:..segments", |segments: Vec<String>| Route::Home {})]
//     Home {},
//     #[route("/blog")]
//     Blog {},
//     #[route("/:..segments")]
//     NotFound { segments: Vec<String> },
// }

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
        MaterialIconStylesheet {
            variant: MaterialIconVariant::Regular
        }
        style { "{STYLE}" }
    })
}
