#![allow(non_snake_case)]

mod app;
mod components;
mod pages;
mod styles;

use app::App;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/src/main.css");
const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Dioxus 0.6 Starting...");
    launch(App);
}