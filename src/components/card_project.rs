use yew::prelude::*;

pub struct ProjectCard {}
impl Component for ProjectCard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
        <>
                    <div class="m-2 flex w-72 lg:w-100 flex-col hover:flex-1 hover:grow hover:flex-initial flex-none hover:ease-in-out">
                        <div class="dropdown">
                        <div tabindex="0" class="bg-opacity-100">
                            <div class="tabs w-full flex-grow-0"><button
                                class="tab tab-lifted tab-active tab-border-none tab-lg flex-1">{"Stats"}</button> <button
                                class="tab tab-lifted tab-border-none tab-lg flex-1">{"Info"}</button> <button
                                class="tab tab-lifted tab-border-none tab-lg flex-1">{"Options"}</button>
                            </div>
                        </div>
                        </div>
                        <div class="bg-base-100 grid w-full flex-grow gap-3 rounded-xl rounded-tl-none p-6 shadow-xl">
                        <div class="flex items-center space-x-2">
                            <div class="dropdown">
                            <div tabindex="0">
                                <div class="online avatar">
                                <div class="mask mask-hexagon bg-base-content h-16 w-16 bg-opacity-10 p-px"><img
                                    src="/tailwind-css-component-profile-5@56w.png" alt="Avatar Tailwind CSS Component"
                                    class="mask mask-hexagon"/>
                                </div>
                                </div>
                            </div>
                            </div>
                            <div>
                                <div class="text-lg font-extrabold">{"Beatrice Thurman"}</div>
                                <div class="text-base-content/70 text-sm">{"220 Followers"}</div>
                            </div>
                        </div>
                        <div class="dropdown">
                            <div tabindex="0">
                                <div class="divider text-base-content/60 m-0">{"Reports"}</div>
                            </div>
                        </div>
                            <div class="text-lg font-extrabold">{"Audience Report"}</div>
                        <div class="grid gap-3">
                            <div class="dropdown dropdown-top">
                            <div tabindex="0">
                                <div class="flex items-center p-1"><span class="text-base-content/70 w-48
                                    text-xs">{"Search Engines"}</span>
                                <progress max="100" class="progress progress-success" value="50"></progress>
                                </div>
                                <div class="flex items-center p-1"><span class="text-base-content/70 w-48
                                    text-xs">{"Direct"}</span> <progress
                                    max="100" class="progress progress-primary" value="30"></progress></div>
                                <div class="flex items-center p-1"><span class="text-base-content/70 w-48
                                    text-xs">{"Social Media"}</span>
                                <progress max="100" class="progress progress-secondary" value="70"></progress>
                                </div>
                                <div class="flex items-center p-1"><span class="text-base-content/70 w-48
                                    text-xs">{"Emails"}</span> <progress
                                    max="100" class="progress progress-accent" value="90"></progress></div>
                                <div class="flex items-center p-1"><span class="text-base-content/70 w-48
                                    text-xs">{"Ad campaigns"}</span>
                                <progress max="100" class="progress progress-warning" value="65"></progress>
                                </div>
                            </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        )
    }
}
