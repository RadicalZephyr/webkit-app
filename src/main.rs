#![windows_subsystem = "windows"]

extern crate signal_hook;
extern crate web_view;

use std::{env, thread};

use signal_hook::iterator::Signals;

use web_view::*;

fn main() {
    let url = env::args()
        .nth(1)
        .unwrap_or_else(|| "http://127.0.0.1/index.html".into());

    let view = web_view::builder()
        .title("")
        .content(Content::Url(url))
        .size(480, 272)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .expect("failed to build webview");

    let handle = view.handle();
    let signals = Signals::new(&[signal_hook::SIGHUP]).expect("failed to register for signals");
    thread::spawn(move || {
        for _ in &signals {
            handle
                .dispatch(|webview| webview.eval("window.location.reload()"))
                .ok();
        }
    });

    view.run().expect("failed to run webview");
}
