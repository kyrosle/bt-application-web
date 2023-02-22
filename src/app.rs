use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
  meta::{trackerinfo::Tracker, TestMeta},
  navbar::NavBar,
};

#[derive(Clone, PartialEq)]
pub struct AppState {
  pub trackers_list: Vec<Tracker>,
  pub change_trackers_list: Callback<usize>,
}

pub struct App {
  state: Rc<AppState>,
}

pub enum AppMsg {
  ChangeTrackersList(usize),
}

impl Component for App {
  type Message = AppMsg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let change_trackers_list = ctx.link().callback(AppMsg::ChangeTrackersList);
    App {
      state: Rc::new(AppState {
        trackers_list: vec![Tracker::create_for_test(), Tracker::create_for_test()],
        change_trackers_list,
      }),
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let state = self.state.clone();
    html! {
      <BrowserRouter>
        <ContextProvider<Rc<AppState>> context={state} >
          <NavBar/>
        </ContextProvider<Rc<AppState>>>
      </BrowserRouter>
    }
  }
}
