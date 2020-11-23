use yew::prelude::*;
use yew_functional::function_component;
use yew_recoil::hooks::{use_recoil_state, use_recoil_value};

use crate::state::{COUNTER, TITLE};

#[function_component(Display)]
pub fn display() -> Html {
    let (count, set_count) = use_recoil_state(&COUNTER);
    let title = use_recoil_value(&TITLE);

    let incr = yew::Callback::from(move |_| set_count(*count + 1));

    html! {
        <div class="text-center">
            <h1 class="text-2xl tracking-tight font-extrabold text-gray-900 sm:text-5xl md:text-6xl text-center">
                <span class="block xl:inline">{"Hello recoil!"}</span>
            </h1>
            <p class="text-base text-gray-500 text-center">
                {title}
            </p>
            <br />
            <button onclick=incr>{"Increment value"} </button>
        </div>
    }
}
