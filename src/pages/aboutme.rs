use yew::prelude::*;
use crate::api;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub profile: api::ProfileGH,
}

pub struct PageAboutMe;

impl Component for PageAboutMe {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _profile = &ctx.props().profile;
        html!(<section>
        </section>)
    }
}
