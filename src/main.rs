use dioxus::prelude::*;

use bars::*;
use components::*;

mod bars;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "flex flex-col max-h-lvh h-dvh",
            TopBar {}
            div { class: "flex-row grow",
                Sidebar {}
                Hero {}
            }
            StatusBar {}
        }
    }
}
