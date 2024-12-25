use dioxus::prelude::*;

#[component]
pub fn StatusBar() -> Element {
    rsx! {
        div {
            id: "topbar",
            class: "w-screen h-10 flex-row sticky bottom-0 z-30 bg-inherit",
            button { class: "p-2", "File" }
            button { class: "p-2 float-right", "Bmarks" }
        }
    }
}
