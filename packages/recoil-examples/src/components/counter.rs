use recoil_yew::hooks::use_recoil_value;
use yew::prelude::*;
use yew_functional::function_component;

use crate::state::COUNTER;

#[function_component(Controls)]
pub fn controls() -> Html {
    let count = use_recoil_value(&COUNTER);
    html! { <div> {format!("Controls {:#?}", count)} </div> }
}
