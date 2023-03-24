use crate::components::media_social::MediaSocials;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PropsAbout {
    pub id: String,
    pub name: String,
    pub short_desc: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct AboutComp {
    pub id: String,
}

impl Component for AboutComp {
    type Message = ();
    type Properties = PropsAbout;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id.to_owned(),
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let prop = ctx.props();
        html! {
          <div id={self.id.to_owned()} class="bg-primary-300">
            <div class="flex flex-col w-full lg:flex-row py-16 md:py-20 items-center">
              <div class="grid flex-grow card rounded-box place-items-center
                    items-center sm:w-3/4 lg:w-3/5 lg:ml-10 mx-10">
                    <h2 class="font-header text-4xl font-semibold uppercase
                        sm:text-5xl md:text-6xl text-center">
                        {"Who am I?"}
                    </h2>
                    <h3 class="pt-6 font-header text-xl font-medium sm:text-2xl
                        md:text-3xl text-center">
                        {"I'm "}{prop.name.to_owned()}<br/>{prop.short_desc.to_owned()}
                    </h3>
                    <div class="flex flex-col justify-center pt-6 sm:flex-row lg:justify-start">
                        <div class="flex items-center justify-center sm:justify-start">
                            <p class="font-body text-lg font-semibold uppercase text-primary">
                                {"Connect with me"} </p>
                                <div class="hidden sm:block">
                                    <i class="bx bx-chevron-right text-2xl text-primary"></i>
                                </div>
                        </div>
                        <MediaSocials/>
                  </div>
                </div>
                <div class="divider lg:divider-horizontal"></div>
              <div class="grid flex-grow card rounded-box pl-0 pt-10 sm:w-3/4 lg:w-2/5
                    lg:pl-12 lg:pt-0 lg:mr-10 mx-10">
                    { prop.children.clone() }
              </div>
            </div>
          </div>
        }
    }
}
