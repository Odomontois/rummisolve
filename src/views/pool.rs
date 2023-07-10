use yew::prelude::*;

use super::{Chosen, Picker};
use crate::model::{self, TilePool};

#[function_component]
pub fn Pool() -> Html {
    let pool = use_state_eq(|| TilePool::default());
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
        <div class ="container" >
            <Picker {on_pick}/>
            <Chosen tiles = {*pool} {on_remove}/>
        </div>
    }
}
