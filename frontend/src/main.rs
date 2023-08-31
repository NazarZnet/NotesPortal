mod app;
mod components;
mod routes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
