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
