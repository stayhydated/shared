mod app;
mod portal;
mod projects;

pub use app::{
    StayhydatedDioxusApp, StayhydatedDocumentAssets, StayhydatedRouterApp, stayhydated_asset_href,
};
pub use portal::{StayhydatedProjectPortal, StayhydatedProjectPortalShell};
pub use projects::{Project, StayhydatedProjectPageMetadata};
pub use stayhydated_dioxus_core::{
    CodeBlock, FullscreenDemoFrame, Href, NavigationTarget, TabContent, TabList, TabTrigger, Tabs,
    TabsOrientation, page_entry_reveal_style, select, surface_reveal_style,
};
