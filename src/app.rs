use wasm_bindgen::prelude::*;
use yew::prelude::*;

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

#[function_component]
pub fn ColorChoose() -> Html {
    let chosen_color = use_state_eq(|| "red".to_string());
    let choose_color = {
        let chosen_color = chosen_color.clone();
        Callback::from(move |s: String| chosen_color.set(s))
    };

    let colors = ["red", "green", "blue", "black"];

    let panels = colors
        .iter()
        .map(|col| {
            html! {
                <ColorPanel
                    choose_color = { choose_color.clone() }
                    color = { col.to_string() }
                    is_chosen = {col == &*chosen_color }
                />
            }
        })
        .collect::<Html>();

    html! {
        <div class="container">
            { panels }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ColorPanelProps {
    choose_color: Callback<String>,
    color: AttrValue,
    is_chosen: bool,
}

#[function_component]
fn ColorPanel(props: &ColorPanelProps) -> Html {
    let ColorPanelProps {
        choose_color,
        color,
        is_chosen,
    } = props.clone();
    let class = classes!(
        "button",
        is_chosen.then(|| "selected"),
        format!("pick-{color}")
    );
    let text = format!("{color}");
    let onclick = Callback::from(move |_| {
        choose_color.emit(color.to_string());
    });
    html! {
        <button {onclick} {class}> { text } </button>
    }
}
