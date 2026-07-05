use stayhydated_dioxus_core::{DisplayText, Href, ProjectMark, ProjectOption};

fn main() {
    let _ = ProjectOption {
        id: "stayhydated",
        mark: ProjectMark::new("SH"),
        name: DisplayText::new("stayhydated"),
        description: None,
        href: Href::new("/"),
    };
}
