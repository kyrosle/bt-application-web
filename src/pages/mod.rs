use yew::prelude::*;

pub mod torrent;
pub mod home;
pub mod info;
pub mod search;

pub use torrent::*;
pub use home::*;
pub use info::*;
pub use search::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! {}
}
