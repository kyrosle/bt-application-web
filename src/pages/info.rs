use yew::prelude::*;

#[function_component(MoreInfo)]
pub fn more_info() -> Html {
  html! {
      <div class="hero min-h-screen" style="background-image: url(public/background.jpg);">
          <div class="hero-overlay bg-opacity-60"></div>
          <div class="hero-content text-center text-neutral-content">
              <div class="max-w-md">
                  <h1 class="mb-5 text-5xl font-bold">{"Hello Guys."}</h1>
                  <p class="mb-5">{"You can get the source code from the github link: "}</p>
                  <a class="mb-6" href="https://github.com/kyrosle/bt-rust">{"bt-rust: https://github.com/kyrosle/bt-rust"}</a>
              </div>
          </div>
      </div>
  }
}
