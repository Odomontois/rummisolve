use yew::prelude::*;

use crate::model;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub tile: model::Tile,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
}

#[function_component]
pub fn Tile(props: &Props) -> Html {
    let Props { tile, on_click } = props.clone();

    match tile {
        model::Tile::Normal { color, value } => {
            let color_name = color.name();
            html! {
                <button onclick={on_click} class={classes!("button", format!("pick-{color_name}"))}> {format!("{value}")} </button>
            }
        }
        model::Tile::Joker => {
            html! {
                <button onclick={on_click} class={classes!("button")}>
                    <span class = "icon">
                        <i class="fa-solid fa-face-smile"></i>
                    </span>
                </button>
            }
        }
    }
}
