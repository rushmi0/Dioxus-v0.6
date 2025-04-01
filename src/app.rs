#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::document::Stylesheet;
use crate::pages::HomePage;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/src/main.css");


#[component]
pub fn App() -> Element {
    rsx! {
        Stylesheet { href: asset!("public/tailwind.css") }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        HomePage {}
    }
}