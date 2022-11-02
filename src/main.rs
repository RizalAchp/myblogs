#![allow(dead_code)]
#![allow(unsafe_code)]
mod api;
mod components;
mod contents;
mod markdown_parser;
mod pages;

use api::{get_languages, request_get, ApiGithub, LangCapability, ProfileGH, RepoGH};
use components::ViewProvider;
use gloo_storage::{LocalStorage, Storage};
use pages::{LandingPage, PageAboutMe, PageNotFound, PageProjects};
use yew::{html::Scope, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageRoute {
    Landing,
    AboutMe,
    Projects,
    Blog,
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Themes {
    Dark(String),
    Light(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    OnToggleTheme,
    OnRouting(PageRoute),
    OnFetchDone(ApiGithub),
    OnFetchOnProfile(ProfileGH),
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub struct App {
    profile: ProfileGH,
    main_route: PageRoute,
    api_data: ApiGithub,
    theme: Themes,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let profile = LocalStorage::get::<ProfileGH>(api::KEY_PROFILE)
            .unwrap_or_else(|_| Self::request_profile(ctx.link()));
        let api_data = LocalStorage::get::<ApiGithub>(api::KEY_REPO)
            .unwrap_or_else(|_| Self::request_repo(ctx.link()));

        Self {
            profile,
            api_data,
            main_route: PageRoute::Landing,
            theme: Themes::Light("lofi".to_owned()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnFetchDone(data) => {
                self.api_data = data;
                LocalStorage::set(api::KEY_REPO, &self.api_data.clone()).ok();
                true
            }
            Msg::OnFetchOnProfile(data) => {
                self.profile = data;
                LocalStorage::set(api::KEY_PROFILE, &self.profile.clone()).ok();
                true
            }
            Msg::OnRouting(route) => {
                self.main_route = route;
                true
            }
            Msg::OnToggleTheme => {
                if self.theme == Themes::Light("lofi".to_owned()) {
                    self.theme = Themes::Dark("black".to_owned())
                } else {
                    self.theme = Themes::Light("lofi".to_owned())
                }
                true
            }
        }
    }
    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::set(api::KEY_REPO, &self.api_data.clone()).ok();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let lang = if !self.api_data.lang_percentage.is_empty() {
            Some(self.api_data.lang_percentage[..5].to_owned())
        } else {
            None
        };
        html! {
        <ViewProvider
            theme={self.theme.clone()}
            callback={ctx.link().callback(|_|Msg::OnToggleTheme)}
        >
            {match self.main_route {
                PageRoute::Landing => yew::html!(
                    <LandingPage
                        langlist={lang}
                        profile={self.profile.clone()}
                        theme={self.theme.clone()}
                    />
                ),
                PageRoute::AboutMe => html!(<PageAboutMe profile={self.profile.clone()}/>),
                PageRoute::Projects => html!(<PageProjects />),
                PageRoute::NotFound => html!(<PageNotFound />),
                PageRoute::Blog => html!()
            }}
        </ViewProvider>
        }
    }
}
impl App {
    fn request_profile(link: &Scope<Self>) -> ProfileGH {
        link.send_future(async {
            Msg::OnFetchOnProfile(
                match request_get::<ProfileGH>("users/RizalAchp".to_owned()).await {
                    Ok(d) => d,
                    Err(_) => Default::default(),
                },
            )
        });
        ProfileGH::default()
    }
    fn request_repo(link: &Scope<Self>) -> ApiGithub {
        link.send_future(async {
            let repository = request_get::<Vec<RepoGH>>("users/RizalAchp/repos".to_owned())
                .await
                .unwrap_or_default();
            let lang_filtered = repository
                .clone()
                .into_iter()
                .filter_map(
                    |RepoGH {
                         languages_url,
                         fork,
                         ..
                     }| {
                        if !fork {
                            Some(languages_url)
                        } else {
                            None
                        }
                    },
                )
                .collect();
            let lang_percentage = get_languages(lang_filtered).await;
            Msg::OnFetchDone(ApiGithub {
                repository,
                lang_percentage,
            })
        });
        ApiGithub::default()
    }
}

fn main() {
    yew::start_app::<App>();
}
