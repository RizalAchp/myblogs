use yew::prelude::*;

use crate::{
    api::{LangCapability, ProfileGH},
    components::{AboutComp, HeroComp, LangComp, ProjectCard},
    Themes,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub langlist: Option<Vec<LangCapability>>,
    pub profile: ProfileGH,
    pub theme: Themes,
}

pub struct LandingPage;

impl Component for LandingPage {
    type Message = ();
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let profile = props.profile;
        let theme = props.theme;
        let langlist = if let Some(lists) = &props.langlist {
            lists
                .clone()
                .iter()
                .enumerate()
                .map(|(idx, lang)| html!(<LangComp {idx} lang={lang.clone()}/>))
                .collect()
        } else {
            html!()
        };
        let hero_img = match theme {
            Themes::Light(_) => "/assets/img/bg-hero-white.jpg",
            Themes::Dark(_) => "/assets/img/bg-hero-black.jpg",
        };
        html!(
            <section>
                <HeroComp
                    id="landing"
                    name={profile.name.clone()}
                    profile_img={profile.avatar_url.clone()}
                    {hero_img}
                />
                <AboutComp id="about" name={profile.name.clone()} short_desc={profile.bio.clone()} >
                    <h1 class="font-header md:text-xl text-md font-bold uppercase">{"Programming Language I Use Often"}</h1>
                    {langlist}
                </AboutComp>
                <div class="divider"></div>
                <div class="place-items-center text-center">
                    <h1 class="font-header font-semibold uppercase sm:text-4xl lg:text-5xl">{"Check Out My Recent Projects!"}</h1>
                </div>
                <div class="flex lg:flex-row items-center py-20 overscroll-x-auto overflow-x-auto lg:flex-nowrap flex-wrap justify-center">
                    <ProjectCard/>
                    <ProjectCard/>
                    <ProjectCard/>
                    <ProjectCard/>
                </div>
            </section>
        )
    }
}
