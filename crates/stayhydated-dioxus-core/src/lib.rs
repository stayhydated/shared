pub mod base;
mod cards;
mod classes;
mod demo_cards;
mod layout;
mod links;
mod metadata;
mod motion;
mod portal;
mod projects;
mod shader_background;
#[cfg(target_arch = "wasm32")]
mod shader_background_renderer;
mod styles;
mod tabs;
mod types;

pub use base::select;
pub use cards::{CodeBlock, FeatureCard};
pub use demo_cards::{DemoCard, DemoCardGrid};
pub use dioxus::prelude::NavigationTarget;
pub use layout::{
    ContributePanelShell, FooterPanel, FullscreenDemoFrame, FullscreenDemoPage, GridColumns,
    HeroListPanel, HeroPanelItem, HeroPanelListKind, ProjectHero, ProjectHomeShell,
    ProjectPageShell, ProjectSurfaceSection,
};
pub use links::{
    ExternalTextLink, ProjectHeroActions, ProjectNavConfig, ProjectNavItem, ProjectNavLabels,
    ProjectNavigationHeader,
};
pub use metadata::{ProjectPageMetadata, project_document_title};
pub use motion::{
    contribute_reveal_style, feature_card_reveal_style, hero_reveal_style, page_entry_reveal_style,
    surface_reveal_style, use_reveal_style,
};
pub use portal::{PortalAccent, PortalDestination, ProjectPortal, ProjectPortalShell};
pub use projects::{ProjectIdentity, ProjectLockup};
pub use styles::{DX_COMPONENTS_THEME_CSS, DX_COMPONENTS_THEME_FILE_NAME, SharedStyles};
pub use tabs::{TabContent, TabList, TabTrigger, Tabs, TabsOrientation};
pub use types::{CssClass, DisplayText, Href, InlineStyle, OptionalDisplayText};
