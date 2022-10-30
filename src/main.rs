#![allow(dead_code)]
mod api;
mod components;
mod contents;
mod markdown_parser;
mod pages;

use crate::markdown_parser::*;
use api::{
    get_languages, request_get, request_get_full, ApiGithub, LangCapability, ProfileGH, RepoGH,
};
use components::landing::Landing;
use components::{about::About, lang::Lang};
use gloo::console::*;
use gloo::storage::{SessionStorage, Storage};
use pages::{PageAboutMe, PageNotFound, PageProjects};

pub enum PageRoute {
    Landing,
    AboutMe,
    Projects,
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
    FetchDone(ApiGithub),
    FetchOnProfile(ProfileGH),
}

pub struct App {
    main_route: PageRoute,
    navbar_active: bool,
    api_data: ApiGithub,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let api_data = SessionStorage::get::<ApiGithub>(api::KEY).unwrap_or_default();
        if api_data.is_empty() {
            ctx.link().send_future(async {
                Msg::FetchOnProfile(
                    match request_get::<ProfileGH>("users/RizalAchp".to_owned()).await {
                        Ok(d) => d,
                        Err(er) => {
                            console_dbg!(format!(
                                "Error: {} on request_get profile",
                                er.to_string()
                            ));
                            Default::default()
                        }
                    },
                )
            });
            ctx.link().send_future(async {
                let repository =
                    match request_get::<Vec<RepoGH>>("users/RizalAchp/repos".to_owned()).await {
                        Ok(d) => d,
                        Err(er) => {
                            console_dbg!(format!("Error: {} on request_get repo", er.to_string()));
                            Default::default()
                        }
                    };
                let lang_percentage = get_languages(
                    repository
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
                        .collect(),
                )
                .await;
                Msg::FetchDone(ApiGithub {
                    repository,
                    lang_percentage,
                    ..Default::default()
                })
            });
        }
        Self {
            main_route: PageRoute::Landing,
            navbar_active: false,
            api_data,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::FetchDone(data) => {
                self.api_data = data;
                SessionStorage::set(api::KEY, self.api_data.clone()).ok();
                true
            }
            Msg::FetchOnProfile(data) => {
                self.api_data.profile = data;
                SessionStorage::set(api::KEY, self.api_data.clone()).ok();
                true
            }
        }
    }
    fn destroy(&mut self, _ctx: &Context<Self>) {
        SessionStorage::set(api::KEY, &self.api_data.clone()).ok();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            { self.view_nav(ctx.link()) }
            <main id="main" class="relative">
                { self.route_main(ctx) }
            </main>
            { self.view_footer(ctx.link()) }
        </>
        }
    }
}

impl App {
    fn route_main(&self, _ctx: &Context<Self>) -> Html {
        match self.main_route {
            PageRoute::Landing => yew::html!(
                <>
                    <Landing id="landing" name={self.api_data.name()}/>
                    <About id="about" name={self.api_data.name()} short_desc={self.api_data.bio()} >
                    <>{ if !self.api_data.is_empty() {
                        html!(for self.api_data
                            .lang_percentage[..5]
                            .iter()
                            .enumerate()
                            .map(|(idx, lang)| html!(<Lang {idx} lang={lang.clone()}/>)))
                        }
                        else {
                            html!()
                        }
                    }</>
                    </About>
                </>
            ),
            PageRoute::AboutMe => yew::html!(<PageAboutMe />),
            PageRoute::Projects => yew::html!(<PageProjects />),
            PageRoute::NotFound => yew::html!(<PageNotFound />),
        }
    }
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let click_callback = link.callback(|_| Msg::ToggleNavbar);
        html! (
        <div class="sticky top-0 z-30 flex h-16 w-full justify-center">
            <div class="drawer">
            <input id="my-drawer-3" type="checkbox" class="drawer-toggle" checked={self.navbar_active} />
            <div class="drawer-content flex flex-col">
              <div class="w-full navbar bg-opacity-90 backdrop-blur transition-all duration-100 bg-base-200 text-base-content shadow-sm">
                <div class="flex-none lg:hidden">
                  <label for="my-drawer-3" class="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" > </path>
                    </svg>
                  </label>
                </div>
                <div class="flex-1 px-2 mx-2"> <a href="#landing"> {"RizalAchp"}</a> </div>
                <div class="flex-none hidden lg:block">
                  <ul class="menu menu-horizontal">
                    <li ><a href="#about">{"About"}</a> </li>
                    <li ><a href="#achievement">{"Achievment"}</a></li>
                    <li ><a href="#blog">{"Blog"}</a></li>
                    <li ><a href="#contact">{"Contact"}</a></li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="drawer-side">
              <label for="my-drawer-3" class="drawer-overlay"> </label>
              <ul class="menu p-4 overflow-y-auto w-80 bg-base-100 backdrop-blur transition-all bg-opacity-90">
                 <li  ><a onclick={&click_callback} href="#about">{"About"}</a> </li>
                 <li ><a onclick={&click_callback} href="#achievement">{"Achievment"}</a></li>
                 <li ><a onclick={&click_callback} href="#blog">{"Blog"}</a></li>
                 <li ><a onclick={click_callback} href="#contact">{"Contact"}</a></li>
              </ul>
            </div>
          </div>
        </div>
        )
    }

    #[inline]
    fn view_footer(&self, _link: &Scope<Self>) -> Html {
        html! {
            <footer class="footer p-10 bg-base-300">
                <div>
                    <svg width="50" height="50" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd" class="fill-current"><path d="M22.672 15.226l-2.432.811.841 2.515c.33 1.019-.209 2.127-1.23 2.456-1.15.325-2.148-.321-2.463-1.226l-.84-2.518-5.013 1.677.84 2.517c.391 1.203-.434 2.542-1.831 2.542-.88 0-1.601-.564-1.86-1.314l-.842-2.516-2.431.809c-1.135.328-2.145-.317-2.463-1.229-.329-1.018.211-2.127 1.231-2.456l2.432-.809-1.621-4.823-2.432.808c-1.355.384-2.558-.59-2.558-1.839 0-.817.509-1.582 1.327-1.846l2.433-.809-.842-2.515c-.33-1.02.211-2.129 1.232-2.458 1.02-.329 2.13.209 2.461 1.229l.842 2.515 5.011-1.677-.839-2.517c-.403-1.238.484-2.553 1.843-2.553.819 0 1.585.509 1.85 1.326l.841 2.517 2.431-.81c1.02-.33 2.131.211 2.461 1.229.332 1.018-.21 2.126-1.23 2.456l-2.433.809 1.622 4.823 2.433-.809c1.242-.401 2.557.484 2.557 1.838 0 .819-.51 1.583-1.328 1.847m-8.992-6.428l-5.01 1.675 1.619 4.828 5.011-1.674-1.62-4.829z"></path></svg>
                    <p>{"ACME Industries Ltd."}<br/>{"Providing reliable tech since 1992}"}</p>
                </div>
                <div>
                    <span class="footer-title">{"Social"}</span>
                    <div class="grid grid-flow-col gap-4">
                        <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"></path></svg></a>
                        <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M19.615 3.184c-3.604-.246-11.631-.245-15.23 0-3.897.266-4.356 2.62-4.385 8.816.029 6.185.484 8.549 4.385 8.816 3.6.245 11.626.246 15.23 0 3.897-.266 4.356-2.62 4.385-8.816-.029-6.185-.484-8.549-4.385-8.816zm-10.615 12.816v-8l8 3.993-8 4.007z"></path></svg></a>
                        <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M9 8h-3v4h3v12h5v-12h3.642l.358-4h-4v-1.667c0-.955.192-1.333 1.115-1.333h2.885v-5h-3.808c-3.596 0-5.192 1.583-5.192 4.615v3.385z"></path></svg></a>
                  </div>
                </div>
            </footer>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
