use yew::prelude::*;

use crate::{
    api::{LangCapability, ProfileGH},
    components::{AboutComp, LangComp, HeroComp},
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
            lists.clone()
                .iter()
                .enumerate()
                .map(|(idx, lang)| html!(<LangComp {idx} lang={lang.clone()}/>)).collect()
        } else {
            html!()
        };
        html!(
            <section>
                <HeroComp
                    id="landing"
                    name={profile.name.clone()}
                    profile_img={profile.avatar_url.clone()}
                    hero_img={match theme {
                        Themes::Light(_) => "/assets/img/bg-hero-white.jpg",
                        Themes::Dark(_) => "/assets/img/bg-hero-black.jpg"
                    }}
                />
                <AboutComp id="about" name={profile.name.clone()} short_desc={profile.bio.clone()} >
                    {langlist}
                </AboutComp>
            </section>
        )
    }
}
