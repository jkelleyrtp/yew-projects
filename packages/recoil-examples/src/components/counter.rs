use yew::prelude::*;
use yew_functional::function_component;
use yew_recoil::hooks::use_recoil_value;

use crate::state::COUNTER;

#[function_component(Controls)]
pub fn controls() -> Html {
    let count = use_recoil_value(&COUNTER);
    html! { <div> {format!("Counter {:#?}", count)} </div> }
}
