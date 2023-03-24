use crate::components::media_social::MediaSocials;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PropsLanding {
    pub id: String,
    pub name: String,
    pub profile_img: String,
    pub hero_img: String,
}

pub struct HeroComp {
    id: String,
}

impl Component for HeroComp {
    type Message = ();
    type Properties = PropsLanding;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name.clone();
        let hero_img = ctx.props().hero_img.clone();
        let profile_img = ctx.props().profile_img.clone();
        html! {
            <section id={self.id.clone()}>
                <div class="hero min-h-screen bg-base-200" style={format!("background-image: url({hero_img});")}>
                    <div class="hero-content flex-col lg:flex-row">
                        <div class="avatar">
                            <div class="md:w-48 w-36 rounded-full">
                                <img src={profile_img} alt="author"/>
                            </div>
                        </div>
                            <div class="pt-8 sm:pt-10 lg:pl-8 lg:pt-0">
                                <h1 class="text-center font-header uppercase font-bold text-5xl md:text-left md:text-6xl">
                                    {"Hello!"}
                                </h1>
                                <h1 class="text-center font-header text-2xl md:text-left md:text-4xl">
                                    {"I'm "}{name}
                                </h1>
                                <div class="flex flex-col justify-center pt-3 sm:flex-row sm:pt-5 lg:justify-start">
                                    <div class="flex items-center justify-center pl-0 sm:justify-start md:pl-1">
                                        <p class="font-body text-lg uppercase">{"Let's connect"}</p>
                                        <div class="hidden sm:block">
                                            <i class="bx bx-chevron-right text-3xl text-yellow"></i>
                                        </div>
                                    </div>
                                <MediaSocials />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
