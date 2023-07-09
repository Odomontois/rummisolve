use yew::prelude::*;

use crate::model::Color;

#[derive(Properties, PartialEq)]
pub struct PickerProps {
    on_pick: Callback<()>,
}

#[function_component]
pub fn Picker() -> Html {
    let chosen_color = use_state_eq(|| Color::Red);
    let chosen_value = use_state_eq(|| 1);
    let choose_color = {
        let chosen_color = chosen_color.clone();
        Callback::from(move |s: Color| chosen_color.set(s))
    };

    let choose_value = {
        let chosen_value = chosen_value.clone();
        Callback::from(move |v: u32| chosen_value.set(v))
    };

    let colors = Color::all();

    let panels = colors
        .map(|col| {
            html! {
                <div class = "level-item">
                    <ColorPick
                        choose_color = { choose_color.clone() }
                        color = { col }
                        is_chosen = {col == *chosen_color }
                    />
                </div>
            }
        })
        .collect::<Html>();

    let values = 1..=13;
    let values = values
        .map(|value| {
            html! {
                <div class = "level-item">
                    <ValuePick
                        {value}
                        color = { *chosen_color }
                        choose_value = { choose_value.clone() }
                    />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="container">
            <div class = "level">
                { panels }
            </div>
            <div class = "level">
                { values }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ValuePickProps {
    value: u32,
    choose_value: Callback<u32>,
    color: Color,
}

#[function_component]
fn ValuePick(props: &ValuePickProps) -> Html {
    let ValuePickProps {
        value,
        choose_value,
        color,
    } = props.clone();

    let onclick = Callback::from(move |_| choose_value.emit(value));
    let color_name = color.name();
    let class = classes!("button", format!("pick-{color_name}"));
    html! {
        <button {onclick} {class}> {format!("{value}")} </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ColorPanelProps {
    choose_color: Callback<Color>,
    color: Color,
    is_chosen: bool,
}

#[function_component]
fn ColorPick(props: &ColorPanelProps) -> Html {
    let ColorPanelProps {
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
