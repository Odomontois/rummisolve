use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::picker::Picker;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <section class= "section is-large">
                <h1 class="title">{ "Rummikub Solver" }</h1>
                <Picker />
            </section>
        </main>
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
