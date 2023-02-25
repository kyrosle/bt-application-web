use std::{collections::HashMap, rc::Rc};

use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
  meta::{trackerinfo::Tracker, TestMeta},
  navbar::NavBar,
};

#[derive(Clone, PartialEq)]
pub struct AppState {
  /// store the trackers globally.
  pub trackers_list: HashMap<String, Tracker>,
  /// a callback function enable the child or grandchild components to change the
  /// trackers_list element above.
  pub change_trackers_list: Callback<String>,
}

pub struct App {
  state: Rc<AppState>,
}

pub enum AppMsg {
  ChangeTrackersList(String),
}

impl Component for App {
  type Message = AppMsg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    // FOR TEST:
    let trackers_list =
      (1..21).into_iter().map(Tracker::create_for_test);

    let trackers_list = trackers_list.into_iter().fold(
      HashMap::new(),
      |mut m, t| {
        m.insert(t.name.clone(), t);
        m
      },
    );

    let change_trackers_list =
      ctx.link().callback(AppMsg::ChangeTrackersList);
    App {
      state: Rc::new(AppState {
        trackers_list,
        change_trackers_list,
      }),
    }
  }

  fn update(
    &mut self,
    _ctx: &Context<Self>,
    msg: Self::Message,
  ) -> bool {
    match msg {
      AppMsg::ChangeTrackersList(name) => {
        let shared_state = Rc::make_mut(&mut self.state);
        shared_state
          .trackers_list
          .get_mut(&name)
          .unwrap()
          .toggle();
        log!(name, "toggle");
        true
      }
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
