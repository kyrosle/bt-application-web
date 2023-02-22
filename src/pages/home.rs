use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
      <div class="hero min-h-screen bg-base-200">
          <div class="hero-content text-center">
              <div class="max-w-md">
                  <h1 class="text-5xl font-bold">{"Hello Guys."}</h1>
                  <p class="py-6">
                      {"
                        This application intends to download file(s) in BitTorrent protocol.
                      "}
                  </p>
                  <button class="btn btn-primary">{"Get Start"}</button>
              </div>
          </div>
      </div>
  }
}
