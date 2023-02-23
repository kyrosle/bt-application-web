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

    // parse the param to every tracker
    // waiting page for the tracker response(some may loss)
    html! {
      <>
      </>
    }
  }
}
