use crate::components::media_social::MediaSocials;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PropsLanding {
    pub id: String,
    pub name: String,
}

pub struct Landing {
    id: String,
}

impl Component for Landing {
    type Message = ();
    type Properties = PropsLanding;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name.clone();
        html! {
        <div id={self.id.clone()}>
          <div class="hero min-h-screen bg-base-200" style="background-image: url(/assets/img/bg-hero-white.jpg);">
            <div class="hero-content flex-col lg:flex-row">
              <div class="rounded-full border-8 shadow-xl">
                <img src="https://avatars.githubusercontent.com/u/81346706" class="h-48 rounded-full sm:h-56" alt="author"/>
              </div>

              <div class="pt-8 sm:pt-10 lg:pl-8 lg:pt-0">
                <h1 class="text-center font-header text-4xl md:text-left sm:text-5xl md:text-6xl">
                  {format!("Hello I'm {name}!")}
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
        </div>
        }
    }
}
