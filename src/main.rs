use dioxus::{desktop::WindowBuilder, prelude::*};
use dioxus_desktop::{use_window, Config};
use tokio::{time::sleep, sync::Mutex };
use std::sync::Arc;
use std::time::Duration;
use serde::{Serialize, Deserialize };

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
    let time_out = 15; // number of seconds before gotcha is displayed
    let mut show_gotcha = use_signal ( || false );
    let mut clicked = use_signal ( || false );
    use_effect(move || {
        // set window to full screen
        let window = use_window();
        window.set_fullscreen(true);
    });

    use_future( move || async move {
        sleep(Duration::from_secs(time_out)).await;
        show_gotcha.set(true);
        sleep(Duration::from_secs(time_out)).await;
        if clicked() {
            // println!("clicked");
            report_to_knowbe4().await;
        }
        let window = use_window();
        window.close();
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
            button {
                onclick: move |_| {
                    show_gotcha.set(true);
                    clicked.set(true);
                },
                "Pay Now"
            }
            if show_gotcha() {
                h1 { "Security Training Alert" }
                p { strong { "this is a simulation" } }
                p {
                    "The phishing email you clicked on recently could have led to "
                     strong {"real ransomware."}
                     " If this had not been a test, your files and company data might now be locked and held for ransom."
                }
                ul {
                    li { "Verify links and sender details before clicking"}
                    li { "Be cautious with unexpected attachments or urgent requests"}
                    li { "Report suspicious emails to Cyber Security immediately" }
                }
                p { "Your awareness is our best defense"}
                button {
                    onclick: move |_| {
                        let window = use_window();
                        window.close();
                    },
                    "Close Window"
                }
            }
        }
    }
}

fn get_useremail () -> String {
    let domain = std::env!("COMPANY_DOMAIN");
    if let Ok(user) = std::env::var("USERNAME") {
        return format!("{}@{}",user,domain).to_string();
    } else {
        return "NONCE".to_string();
    }
}

async fn report_to_knowbe4 () {
    let server = format!("{}/events", std::env!("KNOWBE4_SERVER"));
    let target_user = get_useremail();
    let event_type = "clicked_pay_ransomware_test".to_string();
    let token = std::env!("KNOWBE4_TOKEN");
    let body = Knowbe4Data {target_user, event_type};
    let client = reqwest::Client::new();
    let response = client.post(server)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await.unwrap();
    // println!("status: {}",response.status())
}


#[derive(Serialize, Deserialize, Debug)]
struct Knowbe4Data {
    target_user: String,
    event_type: String
}
