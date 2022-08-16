use crate::contents::AboutMe;
use yew::prelude::*;

pub struct PageAbout {
    aboutme: AboutMe,
}

impl Component for PageAbout{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            aboutme: AboutMe::new(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { aboutme } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &aboutme.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "Interests" }</p>
                                <div class="tags">
                                    { for aboutme.hobby.iter().map(|tag| html! { <span class="tag is-info">{ tag }</span> }) }
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent">
                            <figure class="tile is-child image is-square">
                                <img src={aboutme.image_url.clone()}  alt="The author's profile picture." />
                            </figure>
                        </div>
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ "About me" }</p>
                                    <div class="content">
                                        { "This author has chosen not to reveal anything about themselves" }
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
