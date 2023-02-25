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
      .context::<Rc<AppState>>(
        ctx.link().callback(SliderMsg::ContextChanged),
      )
      .expect("context to be set");
    Self { state, _listener }
  }

  fn update(
    &mut self,
    _ctx: &Context<Self>,
    msg: Self::Message,
  ) -> bool {
    match msg {
      SliderMsg::ContextChanged(state) => {
        self.state = state;
        true
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let trackers_list = self.state.trackers_list.clone();

    let trackers_list = trackers_list
      .into_values()
      .map(|t| {
        t.display(self.state.change_trackers_list.clone())
      })
      .collect::<Vec<_>>();
    html! {
      <ul class="menu p-4 w-80 bg-base-100 text-base-content">
        <div>
          <label>{"Tracker Select"}</label>
          <div style="display: flex; flex-direction: column;height: 75vh; overflow-y: auto; overflow-x: hidden;">
            if trackers_list.is_empty() {
              <label>{"Not Found"}</label>
            } else {
              { for trackers_list }
            }
          </div>
        </div>
      </ul>
    }
  }
}
