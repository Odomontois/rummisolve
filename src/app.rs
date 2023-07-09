use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::color::ColorChoose;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <section class= "section is-large">
                <h1 class="title">{ "Rummikub Solver" }</h1>
                <ColorChoose />
            </section>
        </main>
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
