use dioxus::prelude::*;
use stayhydated_dioxus::{
    DemoGallery, DemoGalleryItem, NavigationTarget, StayhydatedProjectPortalShell,
};

use crate::site::{
    constants::{PROJECT, VERSION, site},
    routing::{AppRoute, PageKind},
};

#[component]
pub(crate) fn DemosPage() -> Element {
    let demos = vec![
        DemoGalleryItem::route(
            crate::site::routing::app_route(PageKind::DioxusDemo),
            "Dioxus",
            "dioxus-demo-card-shader",
        ),
        DemoGalleryItem::route(
            crate::site::routing::app_route(PageKind::TerminalDemo),
            "Terminal",
            "terminal-demo-card-shader",
        ),
        DemoGalleryItem::href(
            site().static_href("bevy-demo"),
            "Bevy UI",
            "bevy-demo-card-shader",
        ),
        DemoGalleryItem::href(
            site().static_href("gpui-demo"),
            "GPUI + gpui-component",
            "gpui-demo-card-shader",
        ),
    ];

    rsx! {
        StayhydatedProjectPortalShell {
            project: PROJECT,
            version: VERSION,
            home: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Home)),
            DemoGallery::<AppRoute> { items: demos }
        }
    }
}
