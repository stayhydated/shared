use crate::components::{FooterPanel, PageHeader};
use crate::site::routing::PageKind;
use dioxus::prelude::*;
use stayhydated_dioxus::{
    DemoCard, DemoCardGrid, GridColumns, PageTitleBand, ProjectPageShell, page_entry_reveal_style,
};

#[component]
pub(crate) fn DemosPage() -> Element {
    let demos_style = page_entry_reveal_style();

    rsx! {
        ProjectPageShell {
            header: rsx!(PageHeader { current_page: PageKind::Demos }),
            footer: Some(rsx!(FooterPanel {})),
            PageTitleBand {
                label: "Demo surfaces",
                title: "Two clients, one AI contract",
                lead: "Use the visual console for product review and the terminal for operator-style inspection. Both clients call the same local library boundary.",
            }
            DemoCardGrid::<crate::site::routing::AppRoute> {
                cards: vec![
                    DemoCard::route(
                        crate::site::routing::app_route(PageKind::DioxusDemo),
                        "Dioxus",
                        "Product console",
                        "Edit operands, reorder the request, and inspect the generated request, response, and trace panels as a buyer-facing workflow.",
                        "Open console",
                    ),
                    DemoCard::route(
                        crate::site::routing::app_route(PageKind::TerminalDemo),
                        "Terminal",
                        "Operator CLI",
                        "Run bracket-list commands through the clap parser and review the same provider-style evidence in a terminal frame.",
                        "Open CLI",
                    ),
                ],
                columns: GridColumns::Two,
                extra_class: "motion-reveal",
                style: demos_style,
            }
        }
    }
}
