#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use component::app_bar::AppBar;
use component::app_content::AppContent;
use dioxus::prelude::*;

mod component;
mod service;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let app_style = r#"
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        text-align: left;
        background-color: #282c34;
    "#;

    cx.render(rsx! {
        div {
            style: "{app_style}",
            AppBar{},
            AppContent{},

            // TODO: remove it after debugging
            // div {
            //     class: "container py-5",
            //     button {
            //         class: "btn btn-primary",
            //         onclick: |_| on_click_button(),
            //         r#type: "button",
            //         "Click Me"
            //     }
            // }
        }
    })
}

#[allow(dead_code)]
async fn on_click_button() {
    let response = reqwest::get("http://127.0.0.1:3000/get")
        .await
        .unwrap()
        .text()
        .await;

    log::info!("{:?}", response);
}
