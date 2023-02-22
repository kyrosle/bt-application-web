use crate::{icons::*, routes::*, slider::Slider};
use gloo::history::{BrowserHistory, History};
use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

pub struct NavBar;
pub enum NavBarMsg {
  ToggleNavbar,
  Search,
}
impl Component for NavBar {
  type Message = NavBarMsg;

  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      NavBarMsg::ToggleNavbar => true,
      NavBarMsg::Search =>  {
        let navigator = ctx.link().navigator().unwrap();
        navigator.push(&Route::Search);
        true
      } ,
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      {self.view_nav(ctx.link())}
    }
  }
}

impl NavBar {
  fn view_nav(&self, link: &Scope<Self>) -> Html {
    // check the url pattern to active current page button.
    let history = BrowserHistory::new();
    let history = history.location();
    let history = history.path();
    
    // select the button active, the pattern should same as the route url pattern.
    let select_index = |pattern: &str| -> Classes {
      if history == pattern {
        classes!("active")
      } else {
        classes!("")
      }
    };
    html! {
      <>
        <div class="drawer" style="height: 90vh;">
          <input id="my-drawer" type="checkbox" class="drawer-toggle" />
          <div class="drawer-content" style="overflow: hidden;">
            <div style="justify-content: flex-start; display: flex; flex-direction: row; align-items: center; margin: 10px 0;">
              <label for="my-drawer" class="btn btn-ghost">
                <MenuIcon/>
              </label>
              <input type="text" placeholder="BitTorrent Search" class="input input-bordered input-sm max-w-xs" />
              <button class="btn btn-ghost btn-xs" style="margin:10px;" onclick={link.callback(|_| NavBarMsg::Search)}>{"Search"}</button>
            </div>
            <div class="navbar bg-base-100">
              <div class="btm-nav" onclick={link.callback(|_| NavBarMsg::ToggleNavbar)}>
                <Link<Route> classes={select_index("/")}  to={Route::Home}>
                  <HomeIcon/>
                  <span class="btm-nav-label">{ "Home" }</span>
                </Link<Route>>
                <Link<Route> classes={select_index("/search")}  to={Route::Search}>
                  <SearchIcon/>
                  <span class="btm-nav-label" >{ "Search" }</span>
                </Link<Route>>
                <Link<Route> classes={select_index("/download")}  to={Route::Download}>
                  <DownloadIcon/>
                  <span class="btm-nav-label">{ "Download" }</span>
                </Link<Route>>
                <Link<Route> classes={select_index("/moreinfo")}  to={Route::MoreInfo}>
                  <MoreInfoIcon/>
                  <span class="btm-nav-label">{ "Info" }</span>
                </Link<Route>>
              </div>
              <Switch<Route> render={switch} />
            </div>
          </div>
          <div class="drawer-side">
            <label for="my-drawer" class="drawer-overlay"></label>
            <Slider/>
          </div>
        </div>
      </>
    }
  }
}
