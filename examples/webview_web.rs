//! Example: Webview Renderer
//! -------------------------
//!
//! This example shows how to use the dioxus_desktop crate to build a basic desktop application.
//!
//! Under the hood, the dioxus_desktop crate bridges a native Dioxus VirtualDom with a custom prebuit application running
//! in the webview runtime. Custom handlers are provided for the webview instance to consume patches and emit user events
//! into the native VDom instance.
//!
//! Currently, NodeRefs won't work properly, but all other event functionality will.

use dioxus::prelude::*;

// #[cfg]
fn main() {
    // env_logger::init();
    dioxus::web::launch(App, |c| c);
}

static App: FC<()> = |cx| {
    dbg!("rednering parent");
    cx.render(rsx! {
        div {
            But {
                h1 {"he"}
            }
            // But {
            //     h1 {"llo"}
            // }
            // But {
            //     h1 {"world"}
            // }
        }
    })
};

static But: FC<()> = |cx| {
    let mut count = use_state(cx, || 0);

    // let d = Dropper { name: "asd" };
    // let handler = move |_| {
    //     dbg!(d.name);
    // };

    cx.render(rsx! {
        div {
            h1 { "Hifive counter: {count}" }
            {cx.children()}
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            // button { onclick: {handler}, "Down low!" }
        }
    })
};

// struct Dropper {
//     name: &'static str,
// }
// impl Drop for Dropper {
//     fn drop(&mut self) {
//         dbg!("dropped");
//     }
// }
