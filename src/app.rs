use crate::pages::*;
use gloo::history::{BrowserHistory, History};
use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

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
pub enum AppMsg {
  ToggleNavbar,
}

pub struct App {}
impl Component for App {
  type Message = AppMsg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    App {}
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      AppMsg::ToggleNavbar => true,
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <BrowserRouter>
        { self.view_nav(ctx.link()) }
      </BrowserRouter>
    }
  }
}
impl App {
  #[inline(never)]
  fn view_nav(&self, link: &Scope<Self>) -> Html {
    let history = BrowserHistory::new();
    let history = history.location();
    let history = history.path();
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
                <svg class="svg-icon" viewBox="0 0 20 20">
                  <path d="M10,1.445c-4.726,0-8.555,3.829-8.555,8.555c0,4.725,3.829,8.555,8.555,8.555c4.725,0,8.555-3.83,8.555-8.555C18.555,5.274,14.725,1.445,10,1.445 M10,17.654c-4.221,0-7.654-3.434-7.654-7.654c0-4.221,3.433-7.654,7.654-7.654c4.222,0,7.654,3.433,7.654,7.654C17.654,14.221,14.222,17.654,10,17.654 M14.39,10c0,0.248-0.203,0.45-0.45,0.45H6.06c-0.248,0-0.45-0.203-0.45-0.45s0.203-0.45,0.45-0.45h7.879C14.187,9.55,14.39,9.752,14.39,10 M14.39,12.702c0,0.247-0.203,0.449-0.45,0.449H6.06c-0.248,0-0.45-0.202-0.45-0.449c0-0.248,0.203-0.451,0.45-0.451h7.879C14.187,12.251,14.39,12.454,14.39,12.702 M14.39,7.298c0,0.248-0.203,0.45-0.45,0.45H6.06c-0.248,0-0.45-0.203-0.45-0.45s0.203-0.45,0.45-0.45h7.879C14.187,6.848,14.39,7.051,14.39,7.298"></path>
              </svg>
              </label>
              <input type="text" placeholder="BitTorrent Search" class="input input-bordered input-sm max-w-xs" />
              <button class="btn btn-ghost btn-xs" style="margin:10px;">{"Search"}</button>
            </div>
            <div class="navbar bg-base-100">
              <div class="btm-nav" onclick={link.callback(|_| AppMsg::ToggleNavbar)}>
                <Link<Route> classes={select_index("/")}  to={Route::Home}>
                    <svg class="svg-icon" viewBox="0 0 20 20">
                    <path d="M18.121,9.88l-7.832-7.836c-0.155-0.158-0.428-0.155-0.584,0L1.842,9.913c-0.262,0.263-0.073,0.705,0.292,0.705h2.069v7.042c0,0.227,0.187,0.414,0.414,0.414h3.725c0.228,0,0.414-0.188,0.414-0.414v-3.313h2.483v3.313c0,0.227,0.187,0.414,0.413,0.414h3.726c0.229,0,0.414-0.188,0.414-0.414v-7.042h2.068h0.004C18.331,10.617,18.389,10.146,18.121,9.88 M14.963,17.245h-2.896v-3.313c0-0.229-0.186-0.415-0.414-0.415H8.342c-0.228,0-0.414,0.187-0.414,0.415v3.313H5.032v-6.628h9.931V17.245z M3.133,9.79l6.864-6.868l6.867,6.868H3.133z"></path>
                  </svg>
                  <span class="btm-nav-label">{ "Home" }</span>
                </Link<Route>>
                <Link<Route> classes={select_index("/search")}  to={Route::Search}>
                  <svg class="svg-icon" viewBox="0 0 20 20">
                  <path d="M18.125,15.804l-4.038-4.037c0.675-1.079,1.012-2.308,1.01-3.534C15.089,4.62,12.199,1.75,8.584,1.75C4.815,1.75,1.982,4.726,2,8.286c0.021,3.577,2.908,6.549,6.578,6.549c1.241,0,2.417-0.347,3.44-0.985l4.032,4.026c0.167,0.166,0.43,0.166,0.596,0l1.479-1.478C18.292,16.234,18.292,15.968,18.125,15.804 M8.578,13.99c-3.198,0-5.716-2.593-5.733-5.71c-0.017-3.084,2.438-5.686,5.74-5.686c3.197,0,5.625,2.493,5.64,5.624C14.242,11.548,11.621,13.99,8.578,13.99 M16.349,16.981l-3.637-3.635c0.131-0.11,0.721-0.695,0.876-0.884l3.642,3.639L16.349,16.981z"></path>
                  </svg>
                  <span class="btm-nav-label">{ "Search" }</span>
                </Link<Route>>
                <Link<Route> classes={select_index("/download")}  to={Route::Download}>
                  <svg class="svg-icon" viewBox="0 0 20 20">
                  <path d="M12.319,5.792L8.836,2.328C8.589,2.08,8.269,2.295,8.269,2.573v1.534C8.115,4.091,7.937,4.084,7.783,4.084c-2.592,0-4.7,2.097-4.7,4.676c0,1.749,0.968,3.337,2.528,4.146c0.352,0.194,0.651-0.257,0.424-0.529c-0.415-0.492-0.643-1.118-0.643-1.762c0-1.514,1.261-2.747,2.787-2.747c0.029,0,0.06,0,0.09,0.002v1.632c0,0.335,0.378,0.435,0.568,0.245l3.483-3.464C12.455,6.147,12.455,5.928,12.319,5.792 M8.938,8.67V7.554c0-0.411-0.528-0.377-0.781-0.377c-1.906,0-3.457,1.542-3.457,3.438c0,0.271,0.033,0.542,0.097,0.805C4.149,10.7,3.775,9.762,3.775,8.76c0-2.197,1.798-3.985,4.008-3.985c0.251,0,0.501,0.023,0.744,0.069c0.212,0.039,0.412-0.124,0.412-0.34v-1.1l2.646,2.633L8.938,8.67z M14.389,7.107c-0.34-0.18-0.662,0.244-0.424,0.529c0.416,0.493,0.644,1.118,0.644,1.762c0,1.515-1.272,2.747-2.798,2.747c-0.029,0-0.061,0-0.089-0.002v-1.631c0-0.354-0.382-0.419-0.558-0.246l-3.482,3.465c-0.136,0.136-0.136,0.355,0,0.49l3.482,3.465c0.189,0.186,0.568,0.096,0.568-0.245v-1.533c0.153,0.016,0.331,0.022,0.484,0.022c2.592,0,4.7-2.098,4.7-4.677C16.917,9.506,15.948,7.917,14.389,7.107 M12.217,15.238c-0.251,0-0.501-0.022-0.743-0.069c-0.212-0.039-0.411,0.125-0.411,0.341v1.101l-2.646-2.634l2.646-2.633v1.116c0,0.174,0.126,0.318,0.295,0.343c0.158,0.024,0.318,0.034,0.486,0.034c1.905,0,3.456-1.542,3.456-3.438c0-0.271-0.032-0.541-0.097-0.804c0.648,0.719,1.022,1.659,1.022,2.66C16.226,13.451,14.428,15.238,12.217,15.238"></path>
                  </svg>
                  <span class="btm-nav-label">{ "Download" }</span>
                </Link<Route>>
                <Link<Route> classes={select_index("/moreinfo")}  to={Route::MoreInfo}>
                  <svg class="svg-icon" viewBox="0 0 20 20">
                  <path d="M9.719,17.073l-6.562-6.51c-0.27-0.268-0.504-0.567-0.696-0.888C1.385,7.89,1.67,5.613,3.155,4.14c0.864-0.856,2.012-1.329,3.233-1.329c1.924,0,3.115,1.12,3.612,1.752c0.499-0.634,1.689-1.752,3.612-1.752c1.221,0,2.369,0.472,3.233,1.329c1.484,1.473,1.771,3.75,0.693,5.537c-0.19,0.32-0.425,0.618-0.695,0.887l-6.562,6.51C10.125,17.229,9.875,17.229,9.719,17.073 M6.388,3.61C5.379,3.61,4.431,4,3.717,4.707C2.495,5.92,2.259,7.794,3.145,9.265c0.158,0.265,0.351,0.51,0.574,0.731L10,16.228l6.281-6.232c0.224-0.221,0.416-0.466,0.573-0.729c0.887-1.472,0.651-3.346-0.571-4.56C15.57,4,14.621,3.61,13.612,3.61c-1.43,0-2.639,0.786-3.268,1.863c-0.154,0.264-0.536,0.264-0.69,0C9.029,4.397,7.82,3.61,6.388,3.61"></path>
                  </svg>
                  <span class="btm-nav-label">{ "Info" }</span>
                </Link<Route>>
              </div>
              <Switch<Route> render={switch} />
            </div>
          </div>
          <div class="drawer-side">
            <label for="my-drawer" class="drawer-overlay"></label>
            <ul class="menu p-4 w-80 bg-base-100 text-base-content">
              <li><a>{"Sidebar Item 1"}</a></li>
              <li><a>{"Sidebar Item 2"}</a></li>
            </ul>
          </div>
        </div>
      </>
    }
  }
}

fn switch(routes: Route) -> Html {
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
