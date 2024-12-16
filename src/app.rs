#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::HomePage;

const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/src/main.css");


#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        HomePage {}
    }
}