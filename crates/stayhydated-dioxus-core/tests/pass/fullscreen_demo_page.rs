use dioxus::prelude::*;
use stayhydated_dioxus_core::{FullscreenDemoFrame, FullscreenDemoPage, LinkTarget};

#[derive(Clone, Debug, PartialEq, Routable)]
enum AppRoute {
    #[route("/demos/", DemosRoute)]
    Demos {},
}

#[component]
fn DemosRoute() -> Element {
    rsx! {}
}

fn frame() -> Element {
    rsx! {
        FullscreenDemoFrame {
            src: "../bevy-demo/",
            title: "Bevy demo",
            allowfullscreen: true,
        }
    }
}

fn page() -> Element {
    rsx! {
        FullscreenDemoPage::<AppRoute> {
            back_target: LinkTarget::route(AppRoute::Demos {}),
            back_label: "Back to demos",
            src: "../gpui-demo/",
            title: "GPUI demo",
        }
    }
}

fn main() {
    let _ = frame;
    let _ = page;
}
