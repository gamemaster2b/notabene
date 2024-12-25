use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    rsx! {
        div {
            id: "topbar",
            class: "w-screen h-10 flex-row sticky top-0 z-30 bg-opacity-80 dark:text-gray-200 dark:bg-opacity-80 border-b dark:border-stone-700 backdrop-blur-xs",
            button { class: "p-2", "File" }
            button { class: "p-2 float-right", "Bmarks" }
        }
    }
}
