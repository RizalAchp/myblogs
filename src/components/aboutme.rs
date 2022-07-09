use yew::prelude::*;
use yew_router::prelude::*;

use crate::contents::{AboutMe, about_me};
use crate::Route;


pub struct AuthorCard {
    author: AboutMe,
}
impl Component for AuthorCard {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            author: about_me()
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.author = about_me();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { author } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img alt="Author's profile picture" src={author.image_url.clone()} />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-3">{ &author.name }</p>
                            <p>
                                { "I like " }
                                <b>{ author.hobby.join(", ") }</b>
                            </p>
                        </div>
                    </div>
                </div>
                <footer class="card-footer">
                    <Link<Route> classes={classes!("card-footer-item")} to={Route::AboutMeDeeper}>
                        { "Profile" }
                    </Link<Route>>
                </footer>
            </div>
        }
    }
}
