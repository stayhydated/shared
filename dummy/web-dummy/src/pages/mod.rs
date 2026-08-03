mod demos;
mod dioxus_demo;
mod home;
mod terminal_demo;

use crate::site::routing::PageKind;
use dioxus::prelude::*;

pub(crate) fn route_content(page: PageKind) -> Element {
    match page {
        PageKind::Home => rsx!(home::HomePage {}),
        PageKind::Demos => rsx!(demos::DemosPage {}),
        PageKind::DioxusDemo => rsx!(dioxus_demo::DioxusDemoPage {}),
        PageKind::TerminalDemo => rsx!(terminal_demo::TerminalDemoPage {}),
    }
}
