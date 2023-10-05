use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::components::home::searchbar::SearchBar;
use crate::Route;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header {
            class: "header--home",
            SearchBar {}
            nav {
                class: "header--home__nav",
                ul {
                    class: "header--home__list",
                    li {
                        Link {
                            to: Route::Settings {},
                            MaterialIcon {
                                name: "settings",
                                size: 24,
                                color: MaterialIconColor::Light,
                            }
                        }
                    }
                }
            }
        }
    })
}
