mod app;
mod views;
mod model;
mod utils;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
