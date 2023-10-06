use dioxus::prelude::*;

pub fn SearchBar(cx: Scope) -> Element {
    cx.render(rsx! {
        select {
            name: "language",
            class: "searchbar",
            option {
                value: "Python",
                class: "searchbar__option",
                "Python"
            }
            option {
                value: "Python",
                class: "searchbar__option",
                "Python"
            }
            option {
                value: "Python",
                class: "searchbar__option",
                "Python"
            }
            option {
                value: "Python",
                class: "searchbar__option",
                "Python"
            }
        }
    })
}
