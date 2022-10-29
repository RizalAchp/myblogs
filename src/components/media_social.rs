use yew::prelude::*;

pub struct MediaSocials;

impl Component for MediaSocials {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
             <div class="flex items-center justify-center pt-5 sm:pt-0 flex-row">
                 <a href="/" class="basis-1/5 p-2 btn btn-circle btn-ghost">
                    <i class="bx bxl-facebook-square text-2xl hover:text-"></i>
                 </a>
                 <a href="/" class="basis-1/5 p-2  btn btn-circle btn-ghost">
                     <i class="bx bxl-twitter text-2xl hover:text-secondary-focus"></i>
                 </a>
                 <a href="/" class="basis-1/5 p-2 btn btn-circle btn-ghost">
                     <i class="bx bxl-dribbble text-2xl hover:text-secondary-focus"></i>
                 </a>
                 <a href="/" class="basis-1/5 p-2 btn btn-circle btn-ghost">
                     <i class="bx bxl-linkedin text-2xl hover:text-secondary-focus"></i>
                 </a>
                 <a href="/" class="basis-1/5 p-2 btn btn-circle btn-ghost">
                     <i class="bx bxl-instagram text-2xl hover:text-secondary-focus"></i>
                 </a>
             </div>
        )
    }
}
