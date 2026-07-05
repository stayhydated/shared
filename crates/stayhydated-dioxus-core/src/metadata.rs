use dioxus::prelude::*;

use crate::DisplayText;

pub fn project_document_title(site_name: impl AsRef<str>, page_title: impl AsRef<str>) -> String {
    let site_name = site_name.as_ref().trim();
    let page_title = page_title.as_ref().trim();

    match (site_name.is_empty(), page_title.is_empty()) {
        (true, true) => String::new(),
        (true, false) => page_title.to_string(),
        (false, true) => site_name.to_string(),
        (false, false) if site_name == page_title => site_name.to_string(),
        (false, false) => format!("{site_name} | {page_title}"),
    }
}

#[component]
pub fn ProjectPageMetadata(
    #[props(into)] site_name: DisplayText,
    #[props(into)] page_title: DisplayText,
    #[props(into)] description: DisplayText,
) -> Element {
    let title = project_document_title(site_name.as_str(), page_title.as_str());
    let description = description.into_string();

    rsx! {
        Title { "{title}" }
        Meta {
            name: "description",
            content: description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_title_includes_home_page_label() {
        assert_eq!(project_document_title("koruma", "Home"), "koruma | Home");
    }

    #[test]
    fn document_title_skips_empty_page_label() {
        assert_eq!(project_document_title("koruma", ""), "koruma");
    }

    #[test]
    fn document_title_avoids_repeating_project_name() {
        assert_eq!(project_document_title("koruma", "koruma"), "koruma");
    }
}
