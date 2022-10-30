use crate::LangCapability;
use yew::prelude::*;

use gloo::timers::callback::Interval;
pub enum Msg {
    OnCount,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LangProp {
    pub idx: usize,
    pub lang: LangCapability,
}
pub struct Lang {
    counter: usize,
    max: usize,
    _interval: Option<Interval>,
}

impl Component for Lang {
    type Message = Msg;
    type Properties = LangProp;
    fn create(ctx: &Context<Self>) -> Self {
        let max = ctx.props().lang.percentage as usize;
        let link = ctx.link().clone();
        let _interval = Some(gloo::timers::callback::Interval::new(100, move || {
            link.send_message(Msg::OnCount)
        }));
        Self {
            counter: 0,
            max,
            _interval,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnCount => {
                if self.counter < self.max {
                    self.counter += 1;
                    true
                } else {
                    if let Some(intv) = &self._interval {
                        drop(intv);
                        self._interval = None;
                    }
                    self._interval = None;
                    false
                }
            }
        }
    }
    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.counter = 0;
        if let Some(intv) = &self._interval {
            drop(intv);
            self._interval = None;
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let prop = ctx.props();
        let idx = prop.idx;
        let name = prop.lang.name.clone();
        html! {
            <div id={format!("lang-{}", idx)} class="pt-6" >
              <div class="flex items-end justify-between">
                <h4 class="font-body font-semibold uppercase text-primary">{name}</h4>
                // <h3 class="font-body text-3xl font-bold text-primary stat-value tabular-nums">{percent}</h3>
                <span class="font-body text-3xl font-bold text-primary count countdown">
                    <span style={format!("--value:{};", self.counter)}></span>{'%'}
                </span>
              </div>
              <progress class="progress progress-primary w-56" value={self.counter.to_string()} max="100"></progress>
            </div>
        }
    }
}
