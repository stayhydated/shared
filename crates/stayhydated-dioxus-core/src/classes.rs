use crate::CssClass;

pub(crate) fn join(base: &str, extra: &CssClass) -> String {
    if extra.is_empty() {
        base.to_owned()
    } else {
        format!("{base} {extra}")
    }
}
