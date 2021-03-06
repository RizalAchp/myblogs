use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod contents;
mod pages;
mod util;
use pages::aboutdeeper::PageAboutDeeper;
use pages::aboutpage::PageAbout;
use pages::home::PageHome;
use pages::notfound::PageNotFound;
use pages::projects::PageProjects;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/projects")]
    Projects,
    #[at("/aboutme")]
    AboutMe,
    #[at("/aboutme/deeper")]
    AboutMeDeeper,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switchs(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <PageHome /> },
        Route::Projects => html! { <PageProjects /> },
        Route::AboutMe => html! { <PageAbout />},
        Route::AboutMeDeeper => html! { <PageAboutDeeper /> },

        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switchs)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and images from " }
                        <a href="https://unsplash.com">{ "Unsplash" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Projects}>
                            { "My Projects" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "More" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::AboutMe}>
                                    { "About Me" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

