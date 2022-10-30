use crate::contents::ContentItem;
use yew::prelude::*;


pub struct ProjectCard {
}
impl Component for ProjectCard{
    type Message = ();
    type Properties = ContentItem;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let project = ctx.props();
        html! (
            <div class="card w-96 glass">
            <figure><img src={project.image.clone()} alt={project.name.clone()}/></figure>
            <div class="card-body">
                <h2 class="card-title">{project.name.clone()}</h2>
                <p>{project.isi.clone()}</p>
                <div class="card-actions justify-end">
                <button class="btn btn-primary">{"Detail"}</button>
                </div>
            </div>
            </div>
        )
    }
}
