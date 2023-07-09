mod app;
mod picker;
mod model;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
