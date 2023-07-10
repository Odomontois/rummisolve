use yew::prelude::*;

use super::{Picker, TileLine, Combinations};
use crate::model::{self, TileSet};

#[function_component]
pub fn Pool() -> Html {
    let pool = use_state_eq(|| TileSet::default());
    let on_pick = {
        let pool = pool.clone();
        Callback::from(move |tile: model::Tile| {
            pool.set(pool.add(tile));
        })
    };
    let on_remove = {
        let pool = pool.clone();
        Callback::from(move |tile: model::Tile| {
            pool.set(pool.remove(tile));
        })
    };
    html! {
        <div class="container">
            <Picker {on_pick} />
            <div class="container">
                <h1 class="title"> {"Chosen Tiles"} </h1>
                <TileLine tiles={*pool} {on_remove} />
            </div>
            <div class="container">
                <Combinations tiles={*pool} />
            </div>
        </div>
    }
}
