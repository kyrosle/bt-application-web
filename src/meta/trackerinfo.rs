use std::str::FromStr;

use url::Url;

use super::TestMeta;

#[derive(Clone, PartialEq)]
pub struct Tracker {
  pub name: String,
  pub url: Url,
}

impl TestMeta for Tracker {
  fn create_for_test() -> Self {
    Tracker {
      name: "Test Tracker".to_string(),
      url: Url::from_str("http://example.com").unwrap(),
    }
  }
}
