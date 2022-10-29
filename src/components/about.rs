use crate::components::media_social::MediaSocials;
use crate::LangCapability;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PropsAbout {
    pub id: String,
    pub name: String,
    pub short_desc: String,
    pub desc: String,
    pub lang: Option<Vec<LangCapability>>,
}

pub struct About {
    pub id: String,
}

impl Component for About {
    type Message = ();
    type Properties = PropsAbout;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id.to_owned(),
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let prop = ctx.props();
        let pct_lang = if let Some(pl) = &prop.lang {
            pl.iter().map(|lc| html!{
              <div class="pt-6">
                  <div class="flex items-end justify-between">
                      <h4 class="font-body font-semibold uppercase text-black"> {lc.name.clone()} </h4>
                      <h3 class="font-body text-3xl font-bold text-primary">{lc.percentage()}</h3>
                  </div>
                  <div class="mt-2 h-3 w-full rounded-full bg-lila">
                      <div class="h-3 rounded-full bg-primary" style={format!("width:{};", lc.percentage())}></div>
                  </div>
              </div>
          }).collect::<Html>()
        } else {
            html!("No Data")
        };
        html! {
          <div id={self.id.to_owned()} class="bg-primary-300">
            <div class="flex flex-col w-full lg:flex-row py-16 md:py-20 ">
              <div class="grid flex-grow card rounded-box place-items-center text-left sm:w-3/4 lg:w-3/5 lg:ml-10 mx-10">
                    <h2 class="font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl">
                        {"Who am I?"}
                    </h2>
                    <h4 class="pt-6 font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl">
                        {format!("I'm {}, {}", prop.name.to_owned(), prop.short_desc.to_owned())}
                    </h4>
                    <p class="pt-6 font-body leading-relaxed text-grey-20">
                        {prop.desc.to_owned()}
                    </p>
                    <div class="flex flex-col justify-center pt-6 sm:flex-row lg:justify-start">
                        <div class="flex items-center justify-center sm:justify-start">
                            <p class="font-body text-lg font-semibold uppercase text-grey-20"> {"Connect with me"} </p>
                                <div class="hidden sm:block">
                                    <i class="bx bx-chevron-right text-2xl text-primary"></i>
                                </div>
                        </div>
                        <MediaSocials/>
                  </div>
                </div>
                <div class="divider lg:divider-horizontal"></div>
              <div class="grid flex-grow card rounded-box pl-0 pt-10 sm:w-3/4 lg:w-2/5 lg:pl-12 lg:pt-0 lg:mr-10 mx-10">
                  {pct_lang}
              </div>
            </div>
          </div>
        }
    }
}
