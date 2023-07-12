use crate::model::debug_info;
use yew::prelude::*;

#[function_component]
pub fn DebugInfo() -> Html {
    html! {
        <div class = "container">
			<h2 class="title">{ "Debug info" }</h2>
            <table class="table is-striped">
            <thead>
                <tr>
                    <th>{ "Name" }</th>
                    <th>{ "Value" }</th>
                </tr>
            </thead>
            <tbody> {
                debug_info().into_iter().map(|[name, v]|
                    html! {
                        <tr>
                            <td>{ name }</td>
                            <td>{ v }</td>
                        </tr>
                    }
                ).collect::<Html>()
            }
            </tbody>
            </table>
        </div>
    }
}
