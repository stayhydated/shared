pub mod base;
mod cards;
mod classes;
mod layout;
mod links;
mod metadata;
mod motion;
mod projects;
mod shader_background;
#[cfg(target_arch = "wasm32")]
mod shader_background_renderer;
mod styles;
mod tabs;
mod types;

pub use base::select;
pub use cards::{CodeBlock, FeatureCard, FeatureCardItem, SectionHeader};
pub use layout::{
    BrandLockup, BrandMark, ButtonLink, ButtonVariant, ContributePanelShell, FooterCopy,
    FooterPanel, FullscreenDemoFrame, FullscreenDemoPage, GridColumns, GridSection, HeaderCluster,
    HeaderNav, Hero, HeroListPanel, HeroPanelItem, HeroPanelListKind, HeroSidePanel,
    PageHeaderShell, PageShell, PageTitleBand, Panel, PanelKind, ProjectHeader, ProjectHero,
    ProjectHomeShell, ProjectPageShell, ProjectSiteHeader, ProjectSurfaceSection,
    SharedGrid as Grid,
};
pub use links::{
    BackLink, ButtonRouteLink, DemoCard, DemoCardGrid, ExternalNavLink, ExternalTextLink,
    LinkTarget, NavLink, ProjectHeroActions, ProjectNav, ProjectNavConfig, ProjectNavHeader,
    ProjectNavItem, ProjectNavLabels, ProjectNavigationHeader, RouteCardLink, RouteLink,
};
pub use metadata::{ProjectPageMetadata, project_document_title};
pub use motion::{
    MotionReveal, contribute_reveal_style, feature_card_reveal_style, hero_reveal_style,
    page_entry_reveal_style, surface_reveal_style, use_reveal_style,
};
pub use projects::{ProjectId, ProjectLockup, ProjectMark, ProjectOption, ProjectSwitcher};
pub use shader_background::{DEFAULT_SHADER_BACKGROUND_CANVAS_ID, ShaderBackground};
pub use styles::{
    DX_COMPONENTS_THEME_CSS, DX_COMPONENTS_THEME_FILE_NAME, DioxusComponentsTheme, SharedStyles,
};
pub use tabs::{TabContent, TabList, TabTrigger, Tabs, TabsOrientation};
pub use types::{CssClass, DisplayText, Href, InlineStyle, OptionalDisplayText};
