use app::App;

mod app;
mod navbar;
mod pages;
mod routes;
mod slider;
mod torrent_engine;

mod icons;
mod meta;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(
    log::Level::Trace,
  ));
  yew::Renderer::<App>::new().render();
}
