use yew::prelude::*;

pub struct Search;

#[derive(Properties, PartialEq)]
pub struct SearchProps {
  pub text: String,
}

pub enum SearchMsg {}

impl Component for Search {
  type Message = SearchMsg;
  type Properties = SearchProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let show = ctx.props().text.clone();
    html! {
      if show.is_empty() {
        <h1>{"Not Search"}</h1>
      } else {
        <h1>{show}</h1>
      }
    }
  }
}
