use app::App;

mod app;
mod navbar;
mod routes;
mod pages;
mod torrent_engine;
mod slider;

mod icons;
mod meta;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
  yew::Renderer::<App>::new().render();
}
