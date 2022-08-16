use crate::contents::ContentItem;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub content: ContentItem,
}

pub struct ContentsCard {
    contents: ContentItem,
}
impl Component for ContentsCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            contents: ctx.props().content.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { contents } = self;
        html! {
            <div class="card">
                <div class="card-content">
                <div class="media">
                    <div class="media-left">
                    <figure class="image is-48x48">
                        <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                    </figure>
                    </div>
                    <div class="media-content">
                    { contents.name.as_ref().unwrap().clone()}
                    <p class="subtitle is-6">{ contents.link.as_ref().unwrap().clone()}</p>
                    </div>
                </div>

                <div class="content">
                {
                    contents.isi.as_ref().unwrap().clone()
                }
                </div>
                </div>
            </div>
        }
    }
}
