#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::NavigationBar;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        NavigationBar {}
    }
}