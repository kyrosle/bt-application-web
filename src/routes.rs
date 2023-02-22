use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/search")]
  Search,
  #[at("/download")]
  Download,
  #[at("/moreinfo")]
  MoreInfo,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => {
      html! { <Home/> }
    }
    Route::Search => {
      html! { <h1>{"Search"}</h1> }
    }
    Route::Download => {
      html! { <Function/> }
    }
    Route::MoreInfo => {
      html! { <MoreInfo/> }
    }
    Route::NotFound => {
      html! { <NotFound/> }
    }
  }
}
