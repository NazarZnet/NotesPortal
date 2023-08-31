mod app;
mod components;
mod routes;
mod api;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::Renderer::<App>::new().render();
}
