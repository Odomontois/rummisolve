mod app;
mod model;
mod views;

use app::App;
use tracing_subscriber::prelude::*;


fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // only partially supported across browsers
        .without_time() // browsers can show timestamps natively in the dev console
        .with_writer(tracing_web::MakeWebConsoleWriter::new());

    tracing_subscriber::registry().with(fmt_layer).init();
    yew::Renderer::<App>::new().render();
}
