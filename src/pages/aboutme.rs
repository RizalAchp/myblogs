use yew::prelude::*;

pub struct PageAboutMe {}

impl Component for PageAboutMe {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!()
    }
}
