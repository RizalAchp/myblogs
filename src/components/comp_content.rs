use crate::contents::Contents;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub content: Contents,
}

pub struct ContentsCard {
    contents: Contents,
}
impl Component for ContentsCard {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let contents = ctx.props().content;
        Self { contents }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { contents } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img alt={contents.imageurl.name.clone()} src={contents.imageurl.link.clone()} />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-3">{ &contents.name.clone() }</p>
                            <p>
                                <b>{ contents.isi.clone() }</b>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
