mod api;
mod app;
mod components;
mod routes;

use app::App;

fn main() {
    dotenv::dotenv().ok();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::Renderer::<App>::new().render();
}
