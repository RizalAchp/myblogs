use yew::prelude::*;

pub struct ProjectCard {}
impl Component for ProjectCard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(<pre><code class=""></code></pre>)
    }
}
