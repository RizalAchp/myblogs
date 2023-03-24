
use super::{Deserialize, Properties, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq,Eq, Deserialize, Serialize, Properties)]
pub struct ProfileGH {
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub repos_url: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub public_repos: u32,
    #[serde(default)]
    pub public_gist: u32,
    #[serde(default)]
    #[serde(flatten)]
    __ignored_fields__: Option<HashMap<String, serde_json::Value>>,
}

impl ProfileGH {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() || self.avatar_url.is_empty()
    }
}
