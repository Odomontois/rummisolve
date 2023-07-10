use yew::prelude::*;

use super::{ColorPick, Tile};
use crate::model::{self, Color, Value};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_pick: Callback<model::Tile>,
}

#[function_component]
pub fn Picker(props: &Props) -> Html {
    let chosen_color = use_state_eq(|| Color::default());
    let choose_color = {
        let chosen_color = chosen_color.clone();
        Callback::from(move |s: Color| chosen_color.set(s))
    };

    let colors = model::Color::all();

    let panels = colors
        .map(|col| {
            html! {
                <div class = "level-item">
                    <ColorPick
                        choose_color = { choose_color.clone() }
                        color = { col }
                        is_chosen = {col == *chosen_color.clone() }
                    />
                </div>
            }
        })
        .collect::<Html>();

    let tiles = Value::all()
        .map(|value| model::Tile::Normal {
            color: *chosen_color.clone(),
            value,
        })
        .chain([model::Tile::Joker]);

    let values = tiles
        .map(|tile| {
            let on_click = {
                let tile = tile.clone();
                let on_pick = props.on_pick.clone();
                Callback::from(move |_| {
                    on_pick.emit(tile.clone());
                })
            };
            html! {
                <div class = "level-item">
                    <Tile {tile} {on_click} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="container">
            <h1 class = "title"> {"Pick a tile"} </h1>
            <div class = "level"> { panels }  </div>
            <div class = "level"> { values }  </div>
        </div>
    }
}

