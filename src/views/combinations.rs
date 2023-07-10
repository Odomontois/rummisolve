use yew::prelude::*;

use super::TileLine;
use crate::model::{all_combos, TileSet};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub tiles: TileSet,
}

#[function_component]
pub fn Combinations(props: &Props) -> Html {
    let Props { tiles } = props.clone();
    let show = use_state_eq(|| false);

    let onclick = {
        let show = show.clone();
        Callback::from(move |_| show.set(!*show))
    };

    let content = if *show {
        let combinations = all_combos().filter(|ts| ts <= &tiles);
        let combinations = combinations.map(|tiles| {
            html! {
                <TileLine {tiles} />
            }
        });
        let combinations = combinations.collect::<Html>();

        html! {
            <div>
                <button class="button is-danger" {onclick}> {"Hide All Combinations"} </button>
                { combinations }
            </div>
        }
    } else {
        html! {
            <button class="button is-primary" {onclick}> {"Show All Combinations"} </button>
        }
    };

    html! {
        <div class="container">
            {content}
        </div>
    }
}
