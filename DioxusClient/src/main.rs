#![allow(non_snake_case)]
use dioxus::prelude::*;
use reqwest::*;
fn main() {
    dioxus_web::launch(App);
}
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
    body {
        match hello_from_rocket(cx).value() {
                Some(Ok(response)) => {rsx!{div { "{response}" }}},
                Some(Err(_)) => rsx!{div { "Trouble getting message from rocket!"}},
                None => rsx!{div{"Waiting for server..."}},
            }
        }
    })
}

fn hello_from_rocket(cx: Scope<'_>) -> &UseFuture<Result<String>> {
    let future = use_future(cx, (), |_| async move {
        match reqwest::get("http://127.0.0.1:8000").await {
            Ok(r) => r,
            Err(e) => {
                println!("{e}");
                panic!("Backend Connection failed")
            }
        }
        .text()
        .await
    });
    future
}
