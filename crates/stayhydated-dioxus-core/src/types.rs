use derive_more::{AsRef, Display, From};

#[derive(AsRef, Clone, Debug, Default, Display, Eq, From, PartialEq)]
#[as_ref(forward)]
#[from(String, &str)]
pub struct DisplayText(String);

impl DisplayText {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptionalDisplayText(Option<DisplayText>);

impl OptionalDisplayText {
    pub fn into_option(self) -> Option<DisplayText> {
        self.0
    }
}

impl From<DisplayText> for OptionalDisplayText {
    fn from(value: DisplayText) -> Self {
        Self(Some(value))
    }
}

impl From<String> for OptionalDisplayText {
    fn from(value: String) -> Self {
        Self(Some(DisplayText::new(value)))
    }
}

impl From<&str> for OptionalDisplayText {
    fn from(value: &str) -> Self {
        Self(Some(DisplayText::new(value)))
    }
}

impl From<Option<DisplayText>> for OptionalDisplayText {
    fn from(value: Option<DisplayText>) -> Self {
        Self(value)
    }
}

impl From<Option<String>> for OptionalDisplayText {
    fn from(value: Option<String>) -> Self {
        Self(value.map(DisplayText::new))
    }
}

impl From<Option<&str>> for OptionalDisplayText {
    fn from(value: Option<&str>) -> Self {
        Self(value.map(DisplayText::new))
    }
}

#[derive(AsRef, Clone, Debug, Display, Eq, From, PartialEq)]
#[as_ref(forward)]
#[from(String, &str)]
pub struct Href(String);

impl Href {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(AsRef, Clone, Debug, Default, Display, Eq, From, PartialEq)]
#[as_ref(forward)]
#[from(String, &str)]
pub struct CssClass(String);

impl CssClass {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(AsRef, Clone, Debug, Default, Display, Eq, From, PartialEq)]
#[as_ref(forward)]
#[from(String, &str)]
pub struct InlineStyle(String);

impl InlineStyle {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}
