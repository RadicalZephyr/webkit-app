#![windows_subsystem = "windows"]

extern crate web_view;

use std::env;

use web_view::*;

fn main() {
    let url = env::args().nth(1).unwrap_or_else(|| "http://127.0.0.1/index.html".into());

    web_view::builder()
        .title("")
        .content(Content::Url(url))
        .size(480, 272)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
