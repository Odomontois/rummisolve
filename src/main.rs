mod app;
mod color;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
