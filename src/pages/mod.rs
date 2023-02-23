use yew::prelude::*;

pub mod home;
pub mod info;
pub mod search;
pub mod torrent;

pub use home::*;
pub use info::*;
pub use search::*;
pub use torrent::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! {}
}
