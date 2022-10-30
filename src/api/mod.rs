mod profile;
mod repo;
mod request;

pub use profile::ProfileGH;
pub use repo::{LangCapability, RepoGH};
pub use request::*;

use gloo::console::{console, console_dbg};
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGithub {
    pub profile: ProfileGH,
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
    pub fn set_lang(&mut self, lang: Vec<LangCapability>) {
        self.lang_percentage = lang;
    }

    #[inline]
    pub fn login(&self) -> String {
        self.profile.login.clone()
    }
    #[inline]
    pub fn avatar_url(&self) -> String {
        self.profile.avatar_url.clone()
    }
    #[inline]
    pub fn name(&self) -> String {
        self.profile.name.clone()
    }
    #[inline]
    pub fn bio(&self) -> String {
        self.profile.bio.clone()
    }
    #[inline]
    pub fn html_url(&self) -> String {
        self.profile.html_url.clone()
    }
    #[inline]
    pub fn public_repos(&self) -> u32 {
        self.profile.public_repos.clone()
    }
    #[inline]
    pub fn public_gist(&self) -> u32 {
        self.profile.public_gist.clone()
    }
}

pub async fn get_languages(repository: Vec<String>) -> Vec<LangCapability> {
    use std::collections::HashMap;
    let mut total: usize = 0;
    let mut all_repo_lang: HashMap<String, usize> = HashMap::new();
    for url in repository {
        match request_get_full::<HashMap<String, usize>>(url.clone()).await {
            Err(e) => {
                console!(format!("Error: {} on api request {}", e.to_string(), url));
                continue;
            }
            Ok(response) => {
                response.into_iter().for_each(|(k, v)| {
                    *all_repo_lang.entry(k).or_insert(0) += v;
                    total += v;
                });
            }
        }
    }
    console_dbg!(&all_repo_lang);
    console_dbg!(&total);
    let mut unsorted = all_repo_lang
        .iter()
        .map(|(k, v)| LangCapability {
            name: k.to_owned(),
            percentage: ((v.clone() as f64 / total as f64) * 100f64) as usize,
        }) // map byte data to percentage
        .collect::<Vec<_>>();
    unsorted.sort_by(|a, b| b.percentage.cmp(&a.percentage));
    console_dbg!(&unsorted);
    unsorted
}

pub const ABOUT_LONG: &str = r"
I am an optimistic, candid, responsible and social person.
I am confident with my thinking analysis that I can convince people with my points.
I am self-reliant, well behaved and above all, a person of strong character.
I take initiative whenever the situation arises and come off with flying colours.
I would like to develop all my existing qualities to the maximum level of perfection,
as such I would like to go for positive experiences in my life because experience is the best teacher of a human being.
";
