use recoil_yew::hooks::{use_recoil_state, use_recoil_value};
use yew::prelude::*;
use yew_functional::function_component;

use crate::state::{COUNTER, TITLE};

#[function_component(Display)]
pub fn display() -> Html {
    let (count, set_count) = use_recoil_state(&COUNTER);
    let title = use_recoil_value(&TITLE);

    let incr = yew::Callback::from(move |_| set_count(*count + 1));

    html! {
        <div>
            {title}
            <br />
            <button onclick=incr>{"Increment value"} </button>
        </div>
    }
}
