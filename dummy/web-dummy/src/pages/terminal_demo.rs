use dioxus::prelude::*;
use stayhydated_dioxus::{NavigationTarget, StayhydatedProjectPortalShell};

use crate::{
    site::{
        constants::{PROJECT, VERSION},
        routing::PageKind,
    },
    terminal,
};

#[component]
pub(crate) fn TerminalDemoPage() -> Element {
    use_effect(move || {
        terminal::launch_terminal_demo();
    });

    rsx! {
        StayhydatedProjectPortalShell {
            project: PROJECT,
            version: VERSION,
            home: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Home)),
            div { class: "demo-page sum-terminal-demo",
                div {
                    id: terminal::TERMINAL_MOUNT_ID,
                    class: "sum-ratzilla-terminal",
                    role: "img",
                    aria_label: "Terminal rendering of the sum-numbers-ai API response",
                    onkeydown: move |event: KeyboardEvent| {
                        trap_terminal_keydown(event);
                    },
                }
            }
        }
    }
}

fn trap_terminal_keydown(event: KeyboardEvent) {
    event.prevent_default();
    event.stop_propagation();
}
