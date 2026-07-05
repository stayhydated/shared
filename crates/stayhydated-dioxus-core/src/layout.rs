use dioxus::prelude::*;
use dioxus_primitives::navbar::{self, NavbarProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::{
    CssClass, DisplayText, Href, InlineStyle, OptionalDisplayText,
    cards::SectionHeader,
    classes,
    links::{BackLink, LinkTarget},
    projects::{ProjectOption, ProjectSwitcher},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelKind {
    Hero,
    Section,
    Code,
    PageTitle,
    Contribute,
}

impl PanelKind {
    fn class(self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::Section => "section-band",
            Self::Code => "code-band",
            Self::PageTitle => "page-title-band",
            Self::Contribute => "contribute-panel",
        }
    }
}

#[component]
pub fn PageShell(children: Element) -> Element {
    rsx! {
        main { class: "page-shell", {children} }
    }
}

#[component]
pub fn ProjectPageShell(
    header: Element,
    children: Element,
    #[props(default)] footer: Option<Element>,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] main_extra_class: CssClass,
) -> Element {
    let shell_class = classes::join("page-shell", &extra_class);
    let main_class = classes::join("stack", &main_extra_class);

    rsx! {
        div { class: shell_class,
            {header}
            main { class: main_class, {children} }
            if let Some(footer) = footer {
                {footer}
            }
        }
    }
}

#[component]
pub fn ProjectHomeShell(header: Element, footer: Element, children: Element) -> Element {
    rsx! {
        ProjectPageShell {
            header,
            footer: Some(footer),
            {children}
        }
    }
}

#[component]
pub fn Panel(
    kind: PanelKind,
    children: Element,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    let class = classes::join(kind.class(), &extra_class);
    let style = style.into_string();
    rsx! {
        section { class, style, {children} }
    }
}

#[component]
pub fn SharedGrid(
    children: Element,
    #[props(default)] columns: Option<GridColumns>,
    #[props(default, into)] extra_class: CssClass,
) -> Element {
    let class = grid_class(columns, &extra_class);

    rsx! {
        div { class, {children} }
    }
}

#[component]
pub fn GridSection(
    children: Element,
    #[props(default)] columns: Option<GridColumns>,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    let class = grid_class(columns, &extra_class);
    let style = style.into_string();

    rsx! {
        section { class, style, {children} }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridColumns {
    Two,
    Three,
}

fn grid_class(columns: Option<GridColumns>, extra_class: &CssClass) -> String {
    match columns {
        Some(GridColumns::Two) => classes::join("grid columns-2", extra_class),
        Some(GridColumns::Three) => classes::join("grid columns-3", extra_class),
        None => classes::join("grid", extra_class),
    }
}

#[component]
pub fn PageHeaderShell(props: NavbarProps) -> Element {
    let base = attributes!(div {
        class: "page-header"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        navbar::Navbar {
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ProjectSiteHeader(project_lockup: Element, children: Element) -> Element {
    rsx! {
        header { class: "page-header",
            {project_lockup}
            HeaderCluster { {children} }
        }
    }
}

#[component]
pub fn ProjectHeader(
    project: ProjectOption,
    children: Element,
    #[props(default)] project_options: Vec<ProjectOption>,
    #[props(default = DisplayText::new("Project selector"), into)] project_label: DisplayText,
    #[props(default = DisplayText::new("Projects"), into)] project_list_label: DisplayText,
) -> Element {
    let _ = project_options;
    let _ = project_list_label;

    rsx! {
        ProjectSiteHeader {
            project_lockup: rsx! {
                ProjectSwitcher {
                    selected: project,
                    projects: Vec::new(),
                    label: project_label,
                }
            },
            {children}
        }
    }
}

#[component]
pub fn HeaderCluster(children: Element) -> Element {
    rsx! {
        div { class: "header-cluster", {children} }
    }
}

#[component]
pub fn HeaderNav(
    children: Element,
    #[props(default = DisplayText::new("Primary navigation"), into)] label: DisplayText,
) -> Element {
    let label = label.into_string();

    rsx! {
        nav { class: "header-nav-links", "aria-label": label, {children} }
    }
}

#[component]
pub fn BrandMark(#[props(into)] label: DisplayText) -> Element {
    rsx! {
        span { class: "brand-mark", "{label}" }
    }
}

#[component]
pub fn BrandLockup(
    #[props(into)] href: Href,
    #[props(into)] mark: DisplayText,
    #[props(into)] kicker: DisplayText,
    #[props(into)] title: DisplayText,
) -> Element {
    rsx! {
        a { class: "brand", href: href.as_str(),
            BrandMark { label: mark }
            span { class: "brand-copy",
                span { class: "brand-kicker", "{kicker}" }
                span { class: "brand-title", "{title}" }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
}

impl ButtonVariant {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

#[component]
pub fn ButtonLink(
    #[props(into)] href: Href,
    #[props(into)] label: DisplayText,
    #[props(default)] variant: ButtonVariant,
) -> Element {
    let variant = variant.class();
    rsx! {
        a { class: format!("button-link {variant}"), href: href.as_str(), "{label}" }
    }
}

#[component]
pub fn Hero(
    children: Element,
    #[props(default)] side: Option<Element>,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    let class = classes::join("hero motion-reveal", &extra_class);
    let style = style.into_string();

    rsx! {
        section { class, style,
            div { class: "hero-copy", {children} }
            if let Some(side) = side {
                {side}
            }
        }
    }
}

#[component]
pub fn ProjectHero(
    #[props(into)] eyebrow: DisplayText,
    #[props(into)] title: DisplayText,
    #[props(into)] body: DisplayText,
    #[props(default)] actions: Option<Element>,
    #[props(default)] side: Option<Element>,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    rsx! {
        Hero {
            side,
            extra_class,
            style,
            div { class: "eyebrow", "{eyebrow}" }
            h1 { "{title}" }
            p { "{body}" }
            if let Some(actions) = actions {
                div { class: "hero-actions", {actions} }
            }
        }
    }
}

#[component]
pub fn HeroSidePanel(
    children: Element,
    #[props(default = CssClass::new("workflow-panel"), into)] class: CssClass,
) -> Element {
    let class = class.into_string();
    rsx! {
        aside { class, {children} }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeroPanelItem {
    pub title: DisplayText,
    pub body: DisplayText,
}

impl HeroPanelItem {
    pub fn new(title: impl Into<DisplayText>, body: impl Into<DisplayText>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum HeroPanelListKind {
    #[default]
    Unordered,
    Ordered,
}

impl HeroPanelListKind {
    pub const fn is_ordered(self) -> bool {
        matches!(self, Self::Ordered)
    }
}

#[component]
pub fn HeroListPanel(
    #[props(into)] label: DisplayText,
    items: Vec<HeroPanelItem>,
    #[props(default)] kind: HeroPanelListKind,
    #[props(default = CssClass::new("workflow-panel"), into)] class: CssClass,
    #[props(default = CssClass::new("workflow-list"), into)] list_class: CssClass,
    #[props(default, into)] body_class: CssClass,
    #[props(default)] label_heading: bool,
) -> Element {
    let list_class = list_class.into_string();
    let body_class = body_class.into_string();

    rsx! {
        HeroSidePanel { class,
            if label_heading {
                h2 { class: "panel-label", "{label}" }
            } else {
                span { class: "panel-label", "{label}" }
            }
            if kind.is_ordered() {
                ol { class: list_class,
                    for (index, item) in items.iter().enumerate() {
                        li { key: "{index}",
                            strong { "{item.title}" }
                            span { class: body_class.clone(), "{item.body}" }
                        }
                    }
                }
            } else {
                ul { class: list_class,
                    for (index, item) in items.iter().enumerate() {
                        li { key: "{index}",
                            strong { "{item.title}" }
                            span { class: body_class.clone(), "{item.body}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProjectSurfaceSection(
    #[props(into)] title: DisplayText,
    children: Element,
    #[props(default, into)] label: OptionalDisplayText,
    #[props(default, into)] lead: OptionalDisplayText,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] heading_extra_class: CssClass,
    #[props(default = CssClass::new("feature-grid"), into)] content_class: CssClass,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    let section_class = classes::join("section-band motion-reveal", &extra_class);
    let heading_class = classes::join(
        "section-heading project-surface-heading",
        &heading_extra_class,
    );
    let content_class = content_class.into_string();
    let style = style.into_string();
    let label = label.into_option();
    let lead = lead.into_option();

    rsx! {
        section { class: section_class, style,
            div { class: "project-surface-header",
                SectionHeader {
                    label,
                    title,
                    lead,
                    class: CssClass::new(heading_class),
                }
            }
            div { class: content_class, {children} }
        }
    }
}

#[component]
pub fn PageTitleBand(
    #[props(default, into)] label: OptionalDisplayText,
    #[props(into)] title: DisplayText,
    #[props(default, into)] lead: OptionalDisplayText,
) -> Element {
    let label = label.into_option();
    let lead = lead.into_option();

    rsx! {
        section { class: "page-title-band motion-reveal",
            if let Some(label) = label {
                span { class: "panel-label", "{label}" }
            }
            h1 { "{title}" }
            if let Some(lead) = lead {
                p { "{lead}" }
            }
        }
    }
}

#[component]
pub fn FooterPanel(children: Element) -> Element {
    rsx! {
        footer { class: "site-footer",
            div { class: "site-footer-shell",
                div { class: "site-footer-sections", {children} }
            }
        }
    }
}

#[component]
pub fn FooterCopy(#[props(into)] label: DisplayText, children: Element) -> Element {
    rsx! {
        p { class: "footer-copy",
            span { class: "footer-label", "{label}" }
            span { class: "footer-text", {children} }
        }
    }
}

#[component]
pub fn ContributePanelShell(
    children: Element,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    let style = style.into_string();
    rsx! {
        section { class: "contribute-panel motion-reveal", style,
            div { class: "contribute-copy", {children} }
        }
    }
}

fn fullscreen_demo_iframe(src: Href, title: DisplayText, allowfullscreen: bool) -> Element {
    rsx! {
        iframe {
            class: "fullscreen-demo-frame",
            src: src.as_str(),
            title: title.as_str(),
            allowfullscreen,
        }
    }
}

#[component]
pub fn FullscreenDemoFrame(
    #[props(into)] src: Href,
    #[props(into)] title: DisplayText,
    #[props(default)] allowfullscreen: bool,
) -> Element {
    rsx! {
        div { class: "fullscreen-demo",
            {fullscreen_demo_iframe(src, title, allowfullscreen)}
        }
    }
}

#[component]
pub fn FullscreenDemoPage<R: Routable + Clone + PartialEq + 'static>(
    back_target: LinkTarget<R>,
    #[props(into)] back_label: DisplayText,
    #[props(into)] src: Href,
    #[props(into)] title: DisplayText,
    #[props(default = true)] allowfullscreen: bool,
) -> Element {
    rsx! {
        div { class: "fullscreen-demo",
            BackLink::<R> {
                target: back_target,
                label: back_label,
            }
            {fullscreen_demo_iframe(src, title, allowfullscreen)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_panel_item_wraps_title_and_body() {
        let item = HeroPanelItem::new("Define", "Attach field validators");

        assert_eq!(item.title.as_str(), "Define");
        assert_eq!(item.body.as_str(), "Attach field validators");
    }

    #[test]
    fn hero_panel_list_kind_reports_ordered_state() {
        assert!(!HeroPanelListKind::Unordered.is_ordered());
        assert!(HeroPanelListKind::Ordered.is_ordered());
    }
}
