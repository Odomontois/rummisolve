use yew::prelude::*;

use crate::model::Color;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub color: Color,
    #[prop_or_default]
    pub choose_color: Callback<Color>,
    #[prop_or_default]
    pub is_chosen: bool,
}

#[function_component]
pub fn ColorPick(props: &Props) -> Html {
    let Props {
        choose_color,
        color,
        is_chosen,
    } = props.clone();
    let color_name = color.name();

    let class = classes!(
        "button",
        is_chosen.then(|| "selected"),
        format!("pick-{color_name}")
    );
    let onclick = Callback::from(move |_| {
        choose_color.emit(color);
    });
    html! {
        <button {onclick} {class}> { color_name } </button>
    }
}
