use app::App;

mod app;
mod pages;
mod utiles;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
  yew::Renderer::<App>::new().render();
}
