use std::rc::Rc;

use yew::prelude::*;

use crate::{app::AppState, meta::trackerinfo::Tracker};

pub struct Search {
  state: Rc<AppState>,
  _listener: ContextHandle<Rc<AppState>>,
}

#[derive(Properties, PartialEq)]
pub struct SearchProps {
  pub text: String,
}

pub enum SearchMsg {
  ContextChanged(Rc<AppState>),
}

impl Component for Search {
  type Message = SearchMsg;
  type Properties = SearchProps;

  fn create(ctx: &Context<Self>) -> Self {
    let (state, _listener) = ctx
      .link()
      .context(
        ctx.link().callback(SearchMsg::ContextChanged),
      )
      .expect("context to be set");

    Self { state, _listener }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let param = ctx.props().text.clone();
    let param =
      percent_encoding::percent_decode(param.as_bytes())
        .decode_utf8()
        .unwrap()
        .to_string();

    let trackers_list = self.state.trackers_list.clone();
    let trackers_list = trackers_list.values().into_iter().map(|t| {
      html! {
        <input type="radio" name="options" data-title={t.name.clone()} class="btn" />
      }
    });

    //  later replace the TrackerSearch Result
    let show_bt = vec![
      html!(<label style="margin: 5px; width: 95vw;">{"SHOW"}</label>);
      100
    ];

    // parse the param to every tracker
    // waiting page for the tracker response(some may loss)
    html! {
      <div style="display: flex; flex-direction: column; width: 95vw;">
        <div class="btn-group" style="display: flex; width: 95vw; overflow-x: auto; border-radius: 5px; border: 2px solid #555; margin: 0 0 5vh 0;">
          {for trackers_list}
        </div>

        <div style="display: flex; overflow-y: auto; flex-direction: column; height: 65vh; margin: 5px;">
          {for show_bt}
        </div>
      </div>
    }
  }
}
