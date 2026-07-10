use crate::components::{FooterPanel, PageHeader};
use crate::site::routing::PageKind;
use dioxus::prelude::*;
use stayhydated_dioxus::{
    ButtonLink, ButtonRouteLink, ButtonVariant, FeatureCard, HeroListPanel, HeroPanelItem,
    HeroPanelListKind, LinkTarget, ProjectHero, ProjectHomeShell, ProjectSurfaceSection,
    feature_card_reveal_style, hero_reveal_style,
};

#[component]
pub(crate) fn HomePage() -> Element {
    let hero_style = hero_reveal_style();

    rsx! {
        ProjectHomeShell {
            header: rsx!(PageHeader { current_page: PageKind::Home }),
            footer: rsx!(FooterPanel {}),
            ProjectHero {
                eyebrow: "sum-numbers-ai-dummy",
                title: "A managed AI addition API",
                body: "sum-numbers-ai-dummy presents integer addition as an auditable AI workflow: structured requests, provider metadata, verified responses, client surfaces, generated docs, and operational evidence.",
                style: hero_style,
                side: Some(rsx! {
                    HeroListPanel {
                        label: "Pitch in one flow",
                        items: vec![
                            HeroPanelItem::new("Accept", "Capture an ordered integer workload with a clear request boundary."),
                            HeroPanelItem::new("Route", "Attach a named endpoint, model, and structured response contract."),
                            HeroPanelItem::new("Verify", "Return the answer with local cross-checks and an audit trail attached."),
                        ],
                        kind: HeroPanelListKind::Ordered,
                        label_heading: true,
                    }
                }),
                actions: Some(rsx! {
                    ButtonLink {
                        href: crate::site::routing::book_href().as_str(),
                        label: "Read the product docs",
                    }
                    ButtonRouteLink::<crate::site::routing::AppRoute> {
                        target: LinkTarget::route(crate::site::routing::app_route(PageKind::Demos)),
                        label: "Open the demos",
                        variant: ButtonVariant::Secondary,
                    }
                }),
            }
            ProjectSurfaceSection {
                label: "Positioning",
                title: "Auditable addition for AI workflow reviews",
                lead: "An addition-focused service keeps the domain model compact and puts contract quality, observability, and client parity at the center.",
                FeatureCard {
                    label: "Buyer story",
                    title: "Show integration discipline",
                    body: "The API demonstrates how AI-backed services can expose inputs, model choice, verification, and evidence in a form reviewers can inspect quickly.",
                    style: feature_card_reveal_style(0),
                }
                FeatureCard {
                    label: "Auditability",
                    title: "Make every provider step inspectable",
                    body: "Each response carries request identity, endpoint, model, latency, token usage, and trace events that explain the provider route.",
                    style: feature_card_reveal_style(1),
                }
                FeatureCard {
                    label: "Client parity",
                    title: "Keep parity easy to prove",
                    body: "The same local library feeds the web console, terminal surface, book examples, sitemap, llms output, and static site build.",
                    style: feature_card_reveal_style(2),
                }
            }
            ProjectSurfaceSection {
                label: "Product surface",
                title: "A focused API with clear operational edges",
                lead: "The API returns structured provider evidence with deterministic local validation, keeping product pages and generated docs aligned.",
                content_class: "feature-grid sum-card-grid",
                FeatureCard {
                    label: "Contract",
                    title: "Explicit request and response types",
                    body: "SumRequest owns operands, endpoint, and model. SumResponse separates the numeric answer from provider metadata and trace events.",
                    style: feature_card_reveal_style(0),
                }
                FeatureCard {
                    label: "Clients",
                    title: "Two clients share the boundary",
                    body: "The Dioxus console and the Ratzilla terminal both call the same local API, which keeps product copy and implementation behavior aligned.",
                    style: feature_card_reveal_style(1),
                }
                FeatureCard {
                    label: "Operations",
                    title: "Provider costs stay visible",
                    body: "Latency and token counts are first-class fields, so docs and demos can discuss cost, routing, and review without external services.",
                    style: feature_card_reveal_style(2),
                }
            }
        }
    }
}
