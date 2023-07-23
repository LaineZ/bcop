use dioxus::prelude::*;

pub fn home(cx: Scope) -> Element {
    cx.render(rsx!(div {
        class: "home",
        h1 {
            "BandcampOnlinePlayer"
        }
    }))
}
