use crate::torrent_engine::TorrentEngine;
use yew::prelude::*;
#[function_component(Function)]
pub fn function() -> Html {
  html! {
      <div class="box" style="margin: 10px">
          <div class="columns is-mobile">
              <div class="column">
                  <TorrentEngine />
              </div>
          </div>
      </div>
  }
}
