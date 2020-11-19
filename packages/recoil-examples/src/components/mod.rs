use recoil_yew::prelude::RecoilRoot;
use yew::prelude::*;
use yew_functional::*;

mod counter;
mod display;
mod textinput;

#[function_component(MyApp)]
pub fn my_app() -> Html {
    html! {
        <RecoilRoot >
            <div>
                <display::Display />
                <textinput::TextBox />
                <counter::Controls />
                <counter::Controls />
            </div>
        </RecoilRoot >
    }
}
