use crate::LangCapability;
use yew::prelude::*;

use gloo::timers::callback::Interval;
enum Msg {
    OnCount,
}
pub struct Lang {
    counter: i32,
    max: i32,
    _interval: Option<Interval>,
}

impl Component for Lang {
    type Message = Msg;
    type Properties = LangCapability;
    fn create(ctx: &Context<Self>) -> Self {
        let max = ctx.props().percentage;
        let link = ctx.link().clone();
        let _interval = Some(gloo::timers::callback::Interval::new(50, move || {
            link.send_message(Msg::OnCount)
        }));
        Self {
            counter: 0,
            max,
            _interval,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnCount => {
                if self.counter < self.max {
                    self.counter += 1;
                    true
                } else {
                    if let Some(int) = self._interval {
                        drop(int);
                    }
                    self._interval = None;
                    false
                }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name;
        html! {
            <div class="pt-6">
              <div class="flex items-end justify-between">
                <h4 class="font-body font-semibold uppercase text-primary">{name}</h4>
                // <h3 class="font-body text-3xl font-bold text-primary stat-value tabular-nums">{percent}</h3>
                <span class="font-body text-3xl font-bold text-primary count countdown">
                    <span style={format!("--value:{};", self.counter)}></span>{'%'}
                </span>
              </div>
              <progress onshow="" class="progress progress-primary w-56" value={self.counter.to_string()} max="100"></progress>
            </div>
        }
    }
}
