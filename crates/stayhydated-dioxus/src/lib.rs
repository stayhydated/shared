mod app;
mod landing;
mod portal;
mod projects;

pub use app::{
    StayhydatedDioxusApp, StayhydatedDocumentAssets, StayhydatedRouterApp, stayhydated_asset_href,
};
pub use landing::StayhydatedProjectLanding;
pub use portal::{StayhydatedProjectPortal, StayhydatedProjectPortalShell};
pub use projects::StayhydatedProjectPageMetadata;
pub use stayhydated_dioxus_core::{
    CodeBlock, DemoCard, DemoCardAccent, FullscreenDemoFrame, Href, LandingLink, LandingTheme,
    NavigationTarget, ProjectLanding, ShaderBackground, TabContent, TabList, TabTrigger, Tabs,
    TabsOrientation, page_entry_reveal_style, select, surface_reveal_style,
};
pub use stayhydated_site::Project;
