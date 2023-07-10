use yew::prelude::*;

use crate::views::Pool;

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <section class= "section is-large">
                <h1 class="title">{ "Game game" }</h1>
                <Pool />
            </section>
        </main>
    }
}


