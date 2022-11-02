mod profile;
mod repo;
mod request;

pub use profile::ProfileGH;
pub use repo::{LangCapability, RepoGH};
pub use request::*;

use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Eq)]
pub struct ApiGithub {
    pub repository: Vec<RepoGH>,
    pub lang_percentage: Vec<LangCapability>,
}

impl ApiGithub {
    pub fn is_empty(&self) -> bool {
        self.lang_percentage.is_empty() || self.repository.is_empty()
    }

    pub fn set_repository(&mut self, repository: Vec<RepoGH>) {
        self.repository = repository;
    }
    #[inline]
    pub fn set_lang(&mut self, lang: Vec<LangCapability>) {
        self.lang_percentage = lang;
    }

}

pub async fn get_languages(repository: Vec<String>) -> Vec<LangCapability> {
    use std::collections::HashMap;
    let mut total: usize = 0;
    let mut all_repo_lang: HashMap<String, usize> = HashMap::new();
    for url in repository {
        match request_get_full::<HashMap<String, usize>>(url.clone()).await {
            Err(_) => {
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
    let mut unsorted = all_repo_lang
        .iter()
        .map(|(k, v)| LangCapability {
            name: k.to_owned(),
            percentage: ((v.clone() as f64 / total as f64) * 100f64) as usize,
        }) // map byte data to percentage
        .collect::<Vec<_>>();
    unsorted.sort_by(|a, b| b.percentage.cmp(&a.percentage));
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
