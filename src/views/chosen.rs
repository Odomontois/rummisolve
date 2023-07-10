use yew::prelude::*;

use super::Tile;
use crate::model::{self, TilePool};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tiles: TilePool,
    #[prop_or_default]
    pub on_remove: Callback<model::Tile>,
}

#[function_component]
pub fn Chosen(props: &Props) -> Html {
    let tiles = props.tiles.tiles();
    let tiles = tiles.map(|tile| {
        let on_click = props.on_remove.reform(move |_| tile.clone());
        html! {
            <Tile {tile} {on_click} />
        }
    });
    let tiles = tiles.collect::<Html>();
    html! {
        <div class = "container">
            <h1 class = "title"> {"Chosen Tiles"} </h1>
            <div class = "buttons">
                {tiles}
            </div>
        </div>
    }
}
