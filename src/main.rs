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
        Router::<Route> { }
        MaterialIconStylesheet {
            variant: MaterialIconVariant::Regular
        }
        style { "{STYLE}" }
    })
}
