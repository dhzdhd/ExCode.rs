use dioxus::prelude::*;

pub fn SearchBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "searchbar",
            h1 {
                "Python"
            }
            div {
                class: "searchbar__dropdown",
                li {
                    ul {
                        "hi"
                    }
                }
            }
        }
    })
}
