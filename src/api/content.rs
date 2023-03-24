use crate::markdown_parser::render_markdown_from_bytes;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use yew::{html, Html};

mod base64 {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::decode(base64.as_bytes()).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(default)]
pub struct ApiContent {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub size: usize,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub git_url: String,
    #[serde(default)]
    pub download_url: String,
    #[serde(default)]
    #[serde(with = "base64")]
    pub content: Vec<u8>,
    #[serde(default)]
    pub encoding: String,
    #[serde(default)]
    #[serde(alias = "type")]
    pub _type: String,
    #[serde(default)]
    pub _links: HashMap<String, serde_json::Value>,
}

impl ApiContent {
    pub fn into_elements(&self) -> Html {
        html!(
            <article class="prose lg:prose-xl dark:prose-invert  prose-img:rounded-xl prose-headings:underline prose-a:text-blue-600">
                { render_markdown_from_bytes(&self.content) }
            </article>
        )
    }
}
