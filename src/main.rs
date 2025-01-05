#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dioxus_app::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Dioxus v0.6 Starting...");
    launch(App);
}