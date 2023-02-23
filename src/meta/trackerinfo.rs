use std::str::FromStr;

use url::Url;

use super::TestMeta;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Tracker {
  pub name: String,
  pub url: Url,
  pub enabled: bool,
}

impl TestMeta for Tracker {
  fn create_for_test(id: usize) -> Self {
    Tracker {
      name: format!("Tracker {}", id),
      url: Url::from_str("http://example.com").unwrap(),
      enabled: false,
    }
  }
}

impl Tracker {
  pub fn display(self, f: Callback<String, ()>) -> Html {
    let name = self.name.clone();
    let onclick = f.reform(move |_| name.clone());

    html! {
      <div style="display:flex; justify-content: space-between; margin: 5px;">
        <label>{&self.name}</label>
        // <label>{&self.url}</label>
        <input type="checkbox" class="toggle" checked={self.enabled} {onclick}/>
      </div>
    }
  }
  pub fn toggle(&mut self) {
    self.enabled = !self.enabled;
  }
}
