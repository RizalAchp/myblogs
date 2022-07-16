use yew::prelude::*;

pub struct PageProjects;

impl Component for PageProjects{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        todo!("implementing projects page")
    }
}
