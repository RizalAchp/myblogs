use yew::prelude::*;

pub struct PageHome;

impl Component for PageHome{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        todo!("implementing home page")
    }
}
