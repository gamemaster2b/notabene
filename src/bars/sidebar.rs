use dioxus::prelude::*;

#[component]
fn SidebarNav() -> Element {
    rsx! {}
}

#[derive(Debug, Clone, PartialEq)]
#[rustfmt::skip]
enum Menu {
    //#[layout(SidebarNav)]
    Explorer {},
    Bookmarks {},
}

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div { class: "flex-2-1 h-auto w-10", id: "sidebar",
            div { class: "float-left flex-col flex-nowrap",
                for i in (1..=30) {
                    button { class: "p-4", "{i}" }
                }
            }
        }
    }
}
