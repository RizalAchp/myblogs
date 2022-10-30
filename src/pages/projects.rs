use crate::components::comp_content::ContentsCard;
use crate::contents::ContentLists;
use yew::prelude::*;

pub struct PageProjects {
    contentlists: ContentLists,
}

impl Component for PageProjects {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let contentlists = ContentLists::parse_contents_from_md();
        Self { contentlists }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { contentlists } = self;
        html! {
            <div class="section container">
                { for contentlists
                    .contents
                    .iter()
                    .map(|content| html!{<ContentsCard content={content.clone()}/>}) }
            </div>
        }
    }
}
