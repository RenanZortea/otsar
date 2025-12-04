// src/main.rs
mod app;
mod components {
    pub mod parser;
}

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
