pub use gloo_net::http::{Request, Response};
pub use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use yew::Properties;

pub const KEY: &str = "rizalachp.data.self";
pub const USERNAME_GITHUB: &str = env!("USERNAME_GITHUB");
pub const API_URL_THIS: &str = "https://api.github.com/repos/{}/myblogs";

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProfileGH {
    pub login: String,
    pub avatar_url: String,
    pub name: String,
    pub bio: String,
    pub html_url: String,
    pub public_repos: u32,
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

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RepoGH {
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub languages_url: String,
    pub contents_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub clone_url: String,
    pub size: u32,

    #[serde(default)]
    #[serde(flatten)]
    __ignored_fields__: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Properties)]
pub struct LangCapability {
    pub name: String,
    pub percentage: i32,
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

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGithub {
    profile: ProfileGH,
    pub repository: Vec<RepoGH>,
    pub lang_percentage: Vec<LangCapability>,
}

impl ApiGithub {
    pub fn is_empty(&self) -> bool {
        self.profile.is_empty() || self.repository.is_empty()
    }

    #[inline]
    pub fn set_profile(&mut self, profile: ProfileGH) {
        self.profile = profile;
    }
    #[inline]
    pub fn set_repository(&mut self, repository: Vec<RepoGH>) {
        self.repository = repository;
    }
    #[inline]
    pub fn login(&self) -> String {
        self.profile.login
    }
    #[inline]
    pub fn avatar_url(&self) -> String {
        self.profile.avatar_url
    }
    #[inline]
    pub fn name(&self) -> String {
        self.profile.name
    }
    #[inline]
    pub fn bio(&self) -> String {
        self.profile.bio
    }
    #[inline]
    pub fn html_url(&self) -> String {
        self.profile.html_url
    }
    #[inline]
    pub fn public_repos(&self) -> u32 {
        self.profile.public_repos
    }
    #[inline]
    pub fn public_gist(&self) -> u32 {
        self.profile.public_gist
    }
}
pub async fn get_languages(repository: Vec<String>) -> Vec<LangCapability> {
    let mut total: usize = 0;
    let mut all_repo_lang: HashMap<String, usize> = HashMap::new();
    for url in repository {
        if let Some(response) = Request::get(&url).send().await.ok() {
            if let Some(maps) = response.json::<HashMap<String, usize>>().await.ok() {
                // insert key and value if not exist
                // or sum the value with the existing value
                maps.into_iter().for_each(|(k, v)| {
                    all_repo_lang
                        .entry(k)
                        .and_modify(|vv| *vv += v)
                        .or_insert(v);
                    total += v;
                });
            };
        }
    }
    let mut unsorted = all_repo_lang
        .into_iter()
        .map(|(k, v)| LangCapability {
            name: k,
            percentage: ((v / total) * 100) as i32,
        }) // map byte data to percentage
        .collect::<Vec<_>>();
    unsorted.sort_by(|a, b| b.percentage.cmp(&a.percentage));
    unsorted
}

pub async fn reqs_profile() -> Option<ProfileGH> {
    let url = format!("https://api.github.com/users/{}", USERNAME_GITHUB);

    match Request::get(&url).send().await {
        Ok(resp) => resp.json::<ProfileGH>().await.ok(),
        Err(e) => None,
    }
}
pub async fn reqs_repos() -> Option<Vec<RepoGH>> {
    let url = format!("https://api.github.com/users/{}/repos", USERNAME_GITHUB);

    match Request::get(&url).send().await {
        Ok(resp) => resp.json::<Vec<RepoGH>>().await.ok(),
        Err(e) => None,
    }
}
