mod app;
mod picker;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
