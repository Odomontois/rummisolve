use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <section class= "section is-large">
                <h1 class="title">{ "Solve Rummikub Situation" }</h1>
                <div class="field">
                    <p class="control">
                    <span class="select">
                        <select>
                            <option>{"Select dropdown"}</option>
                        </select>
                    </span>
                    </p>
            </div>
                <div class="buttons">
                    <a class="button is-primary">{"Primary"}</a>
                    <a class="button is-link">{"Link"}</a>
                </div>
            </section>
        </main>
    }
}
