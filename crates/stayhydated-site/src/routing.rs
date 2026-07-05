use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasePath(String);

impl BasePath {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().trim_matches('/').to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BaseHref(String);

impl BaseHref {
    pub fn root() -> Self {
        Self("/".to_owned())
    }

    pub fn from_base_path(base_path: Option<&BasePath>) -> Self {
        match base_path {
            Some(base_path) if !base_path.as_str().is_empty() => {
                Self(format!("/{}/", base_path.as_str()))
            },
            _ => Self::root(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for BaseHref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutePath(String);

impl RoutePath {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().trim_matches('/').to_owned())
    }

    pub fn root() -> Self {
        Self(String::new())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_output_dir(&self) -> OutputDir {
        OutputDir(self.0.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputDir(String);

impl OutputDir {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().trim_matches('/').to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for OutputDir {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Href(String);

impl Href {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().to_owned())
    }

    pub fn from_route(base_href: &BaseHref, route: &RoutePath) -> Self {
        let base_href = trailing_slash(base_href.as_str());

        if route.is_root() {
            Self(base_href)
        } else {
            Self(format!("{base_href}{}/", route.as_str()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Href {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for Href {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteUrl(String);

impl SiteUrl {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(trailing_slash(value.as_ref()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SiteUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn base_href(base_path: Option<&BasePath>) -> BaseHref {
    BaseHref::from_base_path(base_path)
}

pub fn href(base_href: &BaseHref, route: &RoutePath) -> Href {
    Href::from_route(base_href, route)
}

pub fn site_root_prefix(output_dir: &OutputDir) -> String {
    if output_dir.is_empty() {
        return "./".to_string();
    }

    "../".repeat(
        output_dir
            .as_str()
            .split('/')
            .filter(|segment| !segment.is_empty())
            .count(),
    )
}

pub fn normalized_path_segments<'a>(path: &'a str, base_path: Option<&BasePath>) -> Vec<&'a str> {
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    let base_path_segments = base_path
        .into_iter()
        .flat_map(|base_path| base_path.as_str().split('/'))
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if base_path_segments.is_empty()
        || !segments
            .as_slice()
            .starts_with(base_path_segments.as_slice())
    {
        segments
    } else {
        segments[base_path_segments.len()..].to_vec()
    }
}

fn trailing_slash(value: &str) -> String {
    if value.ends_with('/') {
        value.to_owned()
    } else {
        format!("{value}/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_href_normalizes_optional_base_paths() {
        assert_eq!(base_href(None).as_str(), "/");
        assert_eq!(base_href(Some(&BasePath::new(""))).as_str(), "/");
        assert_eq!(
            base_href(Some(&BasePath::new("/project/"))).as_str(),
            "/project/"
        );
    }

    #[test]
    fn href_joins_routes_under_base_href() {
        let base = BaseHref::from_base_path(Some(&BasePath::new("project")));

        assert_eq!(href(&base, &RoutePath::root()).as_str(), "/project/");
        assert_eq!(
            href(&base, &RoutePath::new("demos")).as_str(),
            "/project/demos/"
        );
        assert_eq!(
            href(&BaseHref::root(), &RoutePath::new("/book/")).as_str(),
            "/book/"
        );
    }

    #[test]
    fn root_prefix_tracks_output_depth() {
        assert_eq!(site_root_prefix(&OutputDir::new("")), "./");
        assert_eq!(site_root_prefix(&OutputDir::new("demos")), "../");
        assert_eq!(site_root_prefix(&OutputDir::new("zh/demos")), "../../");
    }

    #[test]
    fn normalized_segments_strip_matching_base_path() {
        assert_eq!(
            normalized_path_segments("/repo/zh/demos/", Some(&BasePath::new("repo"))),
            ["zh", "demos"]
        );
        assert_eq!(
            normalized_path_segments("/other/zh/demos/", Some(&BasePath::new("repo"))),
            ["other", "zh", "demos"]
        );
    }
}
