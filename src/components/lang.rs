use crate::LangCapability;
use yew::{html::Scope, prelude::*};

use gloo_timers::callback::Interval;
pub enum Msg {
    OnCount,
    OnNewInterval,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LangProp {
    pub idx: usize,
    pub lang: LangCapability,
}
pub struct LangComp {
    counter: usize,
    max: usize,
    _interval: Option<Interval>,
}

impl Component for LangComp {
    type Message = Msg;
    type Properties = LangProp;
    fn create(ctx: &Context<Self>) -> Self {
        let max = ctx.props().lang.percentage as usize;
        Self {
            counter: 0,
            max,
            _interval: None,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
            Msg::OnNewInterval => {
                self._interval = Self::create_interval(ctx.link().clone());
                true
            }
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.counter = 0;
            self._interval = Self::create_interval(ctx.link().clone());
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
            <div onload={ctx.link().callback(|_| Msg::OnNewInterval)} id={format!("lang-{}", idx)} class="pt-6" >
              <div class="flex items-end justify-between">
                <h4 class="font-body font-semibold uppercase text-secondary-100 motion-reduce:hidden">{name}</h4>
                <span class="font-body text-3xl font-bold text-secondary-200 count countdown motion-reduce:hidden">
                    <span style={format!("--value:{};", self.counter)}></span>{'%'}
                </span>
              </div>
              <progress class="progress progress-primary w-56 motion-reduce:hidden text-secondary-100" value={self.counter.to_string()} max="100"></progress>
            </div>
        }
    }
}

impl LangComp {
    #[inline]
    fn create_interval(link: Scope<Self>) -> Option<Interval> {
        Some(Interval::new(150, move || {
            link.send_message(Msg::OnCount)
        }))
    }
}
