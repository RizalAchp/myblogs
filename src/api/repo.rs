use super::{Deserialize, Properties, Serialize};

use std::collections::HashMap;
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Properties, Eq)]
pub struct RepoGH {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub languages_url: String,
    #[serde(default)]
    pub contents_url: String,
    #[serde(default)]
    pub commits_url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub pushed_at: String,
    #[serde(default)]
    pub clone_url: String,
    #[serde(default)]
    pub size: u32,


    #[serde(default)]
    #[serde(flatten)]
    __ignored_fields__: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Properties, Eq)]
pub struct LangCapability {
    pub name: String,
    pub percentage: usize,
}

impl LangCapability {
    #[inline]
    pub fn percentage(&self) -> String {
        format!("{}%", self.percentage)
    }
    #[inline]
    pub fn style(&self) -> String {
        format!("width:{};", self.percentage())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Properties, Eq)]
pub struct RepoDetails {
    repo: RepoGH,

}
