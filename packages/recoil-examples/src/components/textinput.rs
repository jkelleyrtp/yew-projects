use recoil_yew::hooks::use_recoil_state;
use yew::prelude::*;
use yew_functional::function_component;

use crate::state::TITLE;

#[function_component(TextBox)]
pub fn text_box() -> Html {
    let (title, set_title) = use_recoil_state(&TITLE);

    let oninput = Callback::from(move |e: InputData| {
        log::info!("New value is {:#?}", e.value);
        set_title(e.value)
    });

    html! {
        <textarea
            type="text"
            value=&title
            oninput=oninput
        />
    }
}
