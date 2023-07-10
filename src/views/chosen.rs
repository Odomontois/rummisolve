use yew::prelude::*;

use super::Tile;
use crate::model::{self, TileSet};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tiles: TileSet,
    #[prop_or_default]
    pub on_remove: Callback<model::Tile>,
	#[prop_or_default]
	pub disabled: bool,
}

#[function_component]
pub fn TileLine(props: &Props) -> Html {
	let Props{disabled, on_remove, tiles} = props.clone();
    let tiles = tiles.tiles();
    let tiles = tiles.map(|tile| {
        let on_click = on_remove.reform(move |_| tile.clone());
        html! {
            <Tile {tile} {on_click} {disabled}/>
        }
    });
    let tiles = tiles.collect::<Html>();
    html! {
		<div class="buttons">
			{tiles}
		</div>
    }
}
