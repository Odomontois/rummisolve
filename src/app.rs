use yew::prelude::*;

use crate::views::{DebugInfo, Pool};

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <section class= "section is-medium">
                <h1 class="title">{ "Rumikub solver" }</h1>
                <Pool />
            </section>
            <section>
                <DebugInfo />
            </section>
        </main>
    }
}
