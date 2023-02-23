//! Allow user to select tracker webkit which willing to use.
use std::rc::Rc;

use yew::prelude::*;

use crate::{app::AppState, meta::trackerinfo::Tracker};

pub struct Slider {
  state: Rc<AppState>,
  _listener: ContextHandle<Rc<AppState>>,
}

pub enum SliderMsg {
  ContextChanged(Rc<AppState>),
}

impl Component for Slider {
  type Message = SliderMsg;

  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let (state, _listener) = ctx
      .link()
      .context::<Rc<AppState>>(ctx.link().callback(SliderMsg::ContextChanged))
      .expect("context to be set");
    Self { state, _listener }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <ul class="menu p-4 w-80 bg-base-100 text-base-content">
        {self.show_trackers()}
      </ul>
    }
  }
}

impl Slider {
  fn show_trackers(&self) -> Html {
    html! {
      {for self.state.trackers_list.iter().map(show_tracker)}
    }
  }
}

fn show_tracker(tracker: &Tracker) -> Html {
  html! {
    <li>{"Tracker Url here."}</li>
  }
}
